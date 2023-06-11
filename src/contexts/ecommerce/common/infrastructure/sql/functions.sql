CREATE FUNCTION update_timestamp()
    RETURNS TRIGGER AS
    $$
BEGIN
    NEW.__updated_at__ = NOW();
RETURN NEW;
END;
$$
LANGUAGE plpgsql;