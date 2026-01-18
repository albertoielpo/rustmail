#!/bin/bash
HDD=`df -h`
RAM=`free -m`
BODY_MESSAGE=`echo $HDD $RAM | base64 -w 0`
echo $BODY_MESSAGE
curl --location --request POST 'http://localhost:3333/send' --header 'Content-Type: application/json' --data-raw '{
    "mail" : {
        "from": "sender@example.com",
        "to": [
            "receiver@example.com"
        ],
        "subject":  "ciao come va?",
        "text":  "'"$BODY_MESSAGE"'",
        "encoding": "base64"
    }
}'