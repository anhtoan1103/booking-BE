cargo init

cargo add sqlx --features postgres,runtime-tokio,macros
cargo add dotenv
git config core.autocrlf

# test backend API

curl --header "Content-Type: application/json" \
 --request POST \
 --data '{"email":"toto@gmail.com","password":"hehe"}' \
 http://localhost:8080/login
