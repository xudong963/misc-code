-- create table t1, t2
create table t1 (
    t1_id integer,
    t1_name varchar
);


create table t2 (
    t2_id integer,
    t2_num integer,
    t2_name varchar
);

-- insert values

insert into t1 (t1_id, t1_name) values (1, 'a');
insert into t1 (t1_id, t1_name) values (2, 'b');
insert into t1 (t1_id, t1_name) values (3, 'c');

insert into t2 (t2_id, t2_num, t2_name) values (1, 4, 'c');
insert into t2 (t2_id, t2_num, t2_name) values (2, 5, 'd');
insert into t2 (t2_id, t2_num, t2_name) values (3, 6, 'e');


postgres=# select * from t1;
 t1_id | t1_name
-------+---------
     1 | a
     2 | b
     3 | c
(3 rows)

postgres=# select * from t2;
 t2_id | t2_num | t2_name
-------+--------+---------
     1 |      4 | c
     2 |      5 | d
     3 |      6 | e
(3 rows)


-- correlated subquery

select t2.t2_id from t2 where exists (select t1.t1_id from t1 where t1.t1_name = t2.t2_name);

postgres=# select t2.t2_id from t2 where exists (select t1.t1_id from t1 where t1.t1_name = t2.t2_name);
 t2_id
-------
     1
(1 row)

