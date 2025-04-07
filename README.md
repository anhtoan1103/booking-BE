cargo init

cargo add sqlx --features postgres,runtime-tokio,macros
cargo add dotenv
git config core.autocrlf

# test backend API

curl --header "Content-Type: application/json" \
 --request POST \
 --data '{"email":"toto2@gmail.com","password":"hehe"}' \
 http://localhost:8080/login

curl --header "Content-Type: application/json" \
 --request POST \
 --data '{"token":"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIkMmIkMTIkTzhNb1VyMC9hOW9JSldwYi9COUVPZWFBL2dYUURJOFg5N1FXT202Njd4U0ZDdmRIN0FhSC4iLCJjb21wYW55IjoidG90byIsImV4cCI6MTc0NDQwMDQ2MX0.nH_JrH0HEtqoMQdhIhi70uOtj2MezyP3msaPoLQrfsg"}' \
 http://localhost:8080/verify
