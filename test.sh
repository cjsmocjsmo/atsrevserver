# test insertion and retrieval of reviews
curl -d '{"name": "charlie", "email": "boo@gmail.com", "stars": "5", "review": "hellyea"}' -H "Content-Type: application/json" -X POST http://192.168.0.91:8080/insert_rev;
curl -d '{"name": "joette", "email": "joette@gmail.com", "stars": "5", "review": "great job will recommend"}' -H "Content-Type: application/json" -X POST http://192.168.0.91:8080/insert_rev;
curl -d '{"name": "curtis mcbride", "email": "curt@gmail.com", "stars": "5", "review": "hellyea"}' -H "Content-Type: application/json" -X POST http://192.168.0.91:8080/insert_rev;
curl http://192.168.0.91:8080/allrevs;

# test insertion and retrieval of estimates
curl -d '{"name": "charlie", "addr": "924 Hull Ave", "city": "Port Orchard", "phone": "505-555-1234", "email": "boo@gmail.com", "reqservdate": "02/15/2023", "stars": "2", "comment": "job well done hellyea"}' -H "Content-Type: application/json" -X POST http://192.168.0.91:8080/insert_est;
curl -d '{"name": "charlie", "addr": "924 Hull Ave", "city": "Port Orchard", "phone": "505-555-1234", "email": "boo@gmail.com", "reqservdate": "02/15/2023", "stars": "5", "comment": "job well done hellyea"}' -H "Content-Type: application/json" -X POST http://192.168.0.91:8080/insert_est;
curl http://192.168.0.91:8080/allests;
