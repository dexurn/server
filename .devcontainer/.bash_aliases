# Git aliases.
alias gitsetup='git config --global user.name $NAME && git config --global user.email $EMAIL'

# Database
alias dbmate='dbmate --no-dump-schema --migrations-dir /workspace/crates/db/migrations'
alias db='psql $DATABASE_URL'
