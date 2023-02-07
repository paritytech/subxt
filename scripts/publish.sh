#!/usr/bin/env bash
#
# This script is copied from `https://github.com/paritytech/jsonrpsee` with some minor tweaks.
# Add `--dry-run` and/or `--allow-dirty` to your command line to test things before publication.

set -eu

# Find publishable crates by running something like this below and figure out the topological order:
# $ find . -iname 'Cargo.toml' | xargs grep "publish\ *=\ *true" | grep "^.*\.toml" -o
ORDER=(metadata codegen macro subxt cli)

function read_toml () {
	NAME=""
	VERSION=""
	# Extract and parse the "name = ..." line that belongs to the [package] section
	NAME=$(grep -e "\[package\]" -e 'name*=*' ./Cargo.toml | grep -A1 "\[package\]" | tail -n 1 |  sed -e 's/.*"\(.*\)"/\1/')
	VERSION=$(grep "^version" ./Cargo.toml | sed -e 's/.*"\(.*\)"/\1/')
}
function remote_version () {
	REMOTE_VERSION=""
	REMOTE_VERSION=$(cargo search "$NAME" | grep "^$NAME =" | sed -e 's/.*"\(.*\)".*/\1/')
}

# First display the plan
for CRATE_DIR in ${ORDER[@]}; do
	cd $CRATE_DIR > /dev/null
	read_toml
	echo "$NAME@$VERSION"
	cd - > /dev/null
done

read -p ">>>>  Really publish?. Press [enter] to continue. "

set -x

cargo clean

set +x

# Then actually perform publishing.
for CRATE_DIR in ${ORDER[@]}; do
	cd $CRATE_DIR > /dev/null
	read_toml
	remote_version
	# Seems the latest version matches, skip by default.
	if [ "$REMOTE_VERSION" = "$VERSION" ] || [[ "$REMOTE_VERSION" > "$VERSION" ]]; then
		RET=""
		echo "Seems like $NAME@$REMOTE_VERSION is already published. Continuing in 5s. "
		read -t 5 -p ">>>> Type [r][enter] to retry, or [enter] to continue... " RET || true
		if [ "$RET" != "r" ]; then
			echo "Skipping $NAME@$VERSION"
			cd - > /dev/null
			continue
		fi
	fi

	# Attempt to publish (allow retries)
	while : ; do
		# give the user an opportunity to abort or skip before publishing
		echo "🚀 Publishing $NAME@$VERSION..."
		sleep 3

		set +e && set -x
		cargo publish $@
		RES=$?
		set +x && set -e
		# Check if it succeeded
		if [ "$RES" != "0" ]; then
			CHOICE=""
			echo "##### Publishing $NAME failed"
			read -p ">>>>> Type [s][enter] to skip, or [enter] to retry.. " CHOICE
			if [ "$CHOICE" = "s" ]; then
				break
			fi
		else
			break
		fi
	done

	# Wait again to make sure that the new version is published and available.
	echo "Waiting for $NAME@$VERSION to become available at the registry..."
	while : ; do
		sleep 3
		remote_version
		if [ "$REMOTE_VERSION" = "$VERSION" ]; then
			echo "🥳 $NAME@$VERSION published succesfully."
			sleep 3
			break
		else
			echo "#### Got $NAME@$REMOTE_VERSION but expected $NAME@$VERSION. Retrying..."
		fi
	done
	cd - > /dev/null
done

echo "Tagging subxt@$VERSION"
set -x
git tag -s -a v$VERSION -m "Version $VERSION"
sleep 3
git push --tags
