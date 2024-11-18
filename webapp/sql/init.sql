--
DELETE FROM tenant WHERE id > 100;
DELETE FROM visit_history WHERE created_at >= '1654041600';
TRUNCATE TABLE id_generator;
ALTER TABLE id_generator AUTO_INCREMENT=2678400000;
