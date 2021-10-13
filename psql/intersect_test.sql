-- create two test tables
-- foo

create table foo (
    a int,
    b int
);

insert into foo (a, b) values (1, 2);
insert into foo (a, b) values (null, 3);


-- bar
create table bar (
    a int,
    b int
);

insert into bar (a, b) values (1, 2);
insert into bar (a, b) values (null, 3);
insert into bar (a, b) values (3, 4);


-- intersect

select a from foo intersect select a from bar;

/**
 a
---

 1
(2 rows)
**/

-- equivalent transformation ?  since NULL = NULL is not true
select distinct a from foo where a in (select a from bar);

/**
 a
---
 1
(1 row)
**/

