#!/bin/bash
set -e

DATABASE_NAME=ecommerce
DATABASE_TEMPLATE=ecommerce_template

# create databases
createdb -U root $DATABASE_NAME;
createdb -U root $DATABASE_TEMPLATE;

# add extensions
psql -U root -d $DATABASE_NAME     -f /app/src/contexts/ecommerce/common/infrastructure/sql/extensions.sql
psql -U root -d $DATABASE_NAME     -f /app/src/contexts/ecommerce/common/infrastructure/sql/functions.sql

psql -U root -d $DATABASE_TEMPLATE -f /app/src/contexts/ecommerce/common/infrastructure/sql/extensions.sql
psql -U root -d $DATABASE_TEMPLATE -f /app/src/contexts/ecommerce/common/infrastructure/sql/functions.sql

# add schema
psql -U root -d $DATABASE_NAME \
    -f /app/src/contexts/ecommerce/backoffice/infrastructure/sql/product.sql

psql -U root -d $DATABASE_TEMPLATE \
    -f /app/src/contexts/ecommerce/backoffice/infrastructure/sql/product.sql

# add seeds
psql -U root -d $DATABASE_NAME \
    -f /app/src/contexts/ecommerce/backoffice/infrastructure/sql/product_seed.sql
