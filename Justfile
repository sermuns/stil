release:
	cargo release --execute $(git cliff --bumped-version | cut -d'v' -f2)
