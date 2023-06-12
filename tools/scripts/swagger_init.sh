#!/bin/sh

# replace variables programmatically
OAUTH_TOKEN_URL=$OAUTH_TOKEN_URL
OAUTH_AUTHORIZATION_URL=$OAUTH_AUTHORIZATION_URL

cp /openapi.yaml /openapi-tmp.yaml

sed -i "s~__OAUTH_TOKEN_URL__~$OAUTH_TOKEN_URL~g" /openapi-tmp.yaml
sed -i "s~__OAUTH_AUTHORIZATION_URL__~$OAUTH_AUTHORIZATION_URL~g" /openapi-tmp.yaml

cat /openapi-tmp.yaml > /openapi-final.yaml