#!/bin/bash
set -e

SOURCE_ROOT=$SOURCE_ROOT

DATABASE_NAME=ecommerce
DATABASE_TEMPLATE=ecommerce_template

# create databases
createdb -U root $DATABASE_NAME;
createdb -U root $DATABASE_TEMPLATE;

# add extensions
psql -U root -d $DATABASE_NAME     -f "$SOURCE_ROOT/contexts/ecommerce/common/infrastructure/schema/extensions.sql"
psql -U root -d $DATABASE_NAME     -f "$SOURCE_ROOT/contexts/ecommerce/common/infrastructure/schema/functions.sql"

psql -U root -d $DATABASE_TEMPLATE -f "$SOURCE_ROOT/contexts/ecommerce/common/infrastructure/schema/extensions.sql"
psql -U root -d $DATABASE_TEMPLATE -f "$SOURCE_ROOT/contexts/ecommerce/common/infrastructure/schema/functions.sql"

# add schema
psql -U root -d $DATABASE_NAME \
    -f "$SOURCE_ROOT/contexts/ecommerce/backoffice/infrastructure/schema/product.sql"

psql -U root -d $DATABASE_TEMPLATE \
    -f "$SOURCE_ROOT/contexts/ecommerce/backoffice/infrastructure/schema/product.sql"

# add seeds
psql -U root -d $DATABASE_NAME \
    -f "$SOURCE_ROOT/contexts/ecommerce/backoffice/infrastructure/schema/product_seed.sql"
