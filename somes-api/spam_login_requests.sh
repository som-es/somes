for i in $(seq 1 100000);                                                                                                                                                                               
do
    curl --header "Content-Type: application/json" \
        --request POST \
        --data '{"username_or_email":"somes","password":"superdxasdsicher12"}' \
        http://localhost:3000/login
done