-- Add migration script here
-- fansland.tb_test definition

CREATE TABLE Persons (
	id int NOT NULL AUTO_INCREMENT,
	name varchar(255) DEFAULT null,
	primary key(id) using BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 ROW_FORMAT=DYNAMIC COMMENT='积分请求记录';



