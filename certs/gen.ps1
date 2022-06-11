$server = "test"

Set-Alias -Name openssl -Value /usr/local/opt/openssl/bin/openssl
# brew install openssl # if you don't have newest openssl installed

# ca.pri.pem
openssl genrsa -out ca.pri.pem

# server.pri.pem
openssl genrsa -out "${server}.pri.pem"

Write-Output "CA CERTIFICATE"
# ca.cert.pem
openssl req -x509 -new -nodes -key ca.pri.pem -days 3650 -out ca.cert.pem

Write-Output "SERVER CERTIFICATE REQUEST"
# server.req.pem
openssl req -new -nodes -key "${server}.pri.pem" -out "${server}.req.pem"

# server.cert.pem
openssl x509 -req -in "${server}.req.pem" -CA ca.cert.pem -CAkey ca.pri.pem -out "${server}.cert.pem" -days 3650 -extfile "${server}.san.cnf"
