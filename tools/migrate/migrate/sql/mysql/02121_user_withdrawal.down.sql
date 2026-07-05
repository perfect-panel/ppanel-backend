DROP TABLE IF EXISTS `withdrawals`;

DELETE FROM `system`
WHERE `category` = 'invite'
  AND `key` = 'WithdrawalMethod';
