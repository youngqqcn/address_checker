-- Add migration script here

CREATE TABLE address_balance(
	`id` int NOT NULL AUTO_INCREMENT,
    `chain` varchar(20) NOT NULL COMMENT '链名: ETH, BSC, TRON',
    `token` varchar(10) NOT NULL COMMENT 'Token: ETH, BNB, TRX, ERC20_USDT, TRC20_USDT, BEP20_USDT,',
	`addr` varchar(50) NOT NULL,
    `base_balance` varchar(50)  DEFAULT "0",
    `usdt_balance` varchar(50) DEFAULT "0",
    `checked` tinyint(1) NOT NULL DEFAULT 0,
    `update_time` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
	primary key(id) using BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 ROW_FORMAT=DYNAMIC COMMENT='地址余额记录表';




