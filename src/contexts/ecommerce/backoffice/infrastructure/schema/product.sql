CREATE TABLE product
(
    id       UUID DEFAULT uuid_generate_v4(),
    name     TEXT    NOT NULL,
    price    INTEGER NOT NULL,
    currency TEXT    NOT NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    PRIMARY KEY (id)
);

CREATE INDEX products_by_name ON product (name);
CREATE INDEX products_by_currency ON product (currency);

CREATE TRIGGER update_product_timestamp_trigger
    BEFORE UPDATE
    ON product
    FOR EACH ROW
    EXECUTE FUNCTION update_timestamp();