-- create tpch tables
CREATE TABLE IF NOT EXISTS nation
(
    n_nationkey  INTEGER not null,
    n_name       VARCHAR not null,
    n_regionkey  INTEGER not null,
    n_comment    VARCHAR
);

CREATE TABLE IF NOT EXISTS region
(
    r_regionkey  INTEGER not null,
    r_name       VARCHAR not null,
    r_comment    VARCHAR
);

CREATE TABLE IF NOT EXISTS part
(
    p_partkey     BIGINT not null,
    p_name        VARCHAR not null,
    p_mfgr        VARCHAR not null,
    p_brand       VARCHAR not null,
    p_type        VARCHAR not null,
    p_size        INTEGER not null,
    p_container   VARCHAR not null,
    p_retailprice numeric not null,
    p_comment     VARCHAR not null
);

CREATE TABLE IF NOT EXISTS supplier
(
    s_suppkey     BIGINT not null,
    s_name        VARCHAR not null,
    s_address     VARCHAR not null,
    s_nationkey   INTEGER not null,
    s_phone       VARCHAR not null,
    s_acctbal     numeric not null,
    s_comment     VARCHAR not null
);

CREATE TABLE IF NOT EXISTS partsupp
(
    ps_partkey     BIGINT not null,
    ps_suppkey     BIGINT not null,
    ps_availqty    BIGINT not null,
    ps_supplycost  numeric  not null,
    ps_comment     VARCHAR not null
);

CREATE TABLE IF NOT EXISTS customer
(
    c_custkey     BIGINT not null,
    c_name        VARCHAR not null,
    c_address     VARCHAR not null,
    c_nationkey   INTEGER not null,
    c_phone       VARCHAR not null,
    c_acctbal     numeric   not null,
    c_mktsegment  VARCHAR not null,
    c_comment     VARCHAR not null
);

CREATE TABLE IF NOT EXISTS orders
(
    o_orderkey       BIGINT not null,
    o_custkey        BIGINT not null,
    o_orderstatus    VARCHAR not null,
    o_totalprice     numeric not null,
    o_orderdate      DATE not null,
    o_orderpriority  VARCHAR not null,  
    o_clerk          VARCHAR not null, 
    o_shippriority   INTEGER not null,
    o_comment        VARCHAR not null
);

CREATE TABLE IF NOT EXISTS lineitem
(
    l_orderkey    BIGINT not null,
    l_partkey     BIGINT not null,
    l_suppkey     BIGINT not null,
    l_linenumber  BIGINT not null,
    l_quantity    numeric not null,
    l_extendedprice  numeric not null,
    l_discount    numeric not null,
    l_tax         numeric not null,
    l_returnflag  VARCHAR not null,
    l_linestatus  VARCHAR not null,
    l_shipdate    DATE not null,
    l_commitdate  DATE not null,
    l_receiptdate DATE not null,
    l_shipinstruct VARCHAR not null,
    l_shipmode     VARCHAR not null,
    l_comment      VARCHAR not null
);
