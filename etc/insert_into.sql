INSERT INTO Staff VALUES
  (1, 'Alex', 55233066, '3676 Boone Street', 'Warehouseman'),
  (2, 'Steve', 77897884, '304 Doctors Drive', 'Selling'),
  (3, 'Joe', 83562380, '2110 Willow Greene', 'Purchasing'),
  (4, 'Jose', 50060114, '4709 Vine Street', 'Purchasing'),
  (5, 'Parker', 98350668, '3301 Hilltop Street', 'Warehouseman'),
  (6, 'Taylor', 69106817, '3419 Goldcliff Circle', 'Selling'),
  (7, 'Cook', 18269011, '3656 Juniper Drive', 'Selling'),
  (8, 'Wood', 84277362, '1659 Millbrook Road', 'Purchasing');

INSERT INTO Warehouse VALUES
  (1, 'Warehouse 2', '3600 Boone Street', 1),
  (2, 'Warehouse 3', '3201 Hilltop Street', 5);

INSERT INTO Customer VALUES
  (1, 'Mission Realty', 65736495, 'Edward', '4028 Eva Pearl Street', 62380),
  (2, 'Pup N Taco', 51615741, 'Rios', '3248 Tecumsah Lane', 60114),
  (3, 'Gino''s Hamburgers', 87033825, 'Dowell', '1726 Andy Street', 66851),
  (4, 'Weatherill''s', 69975158, 'Shane', '3286 Friendship Lane', 68170),
  (5, 'Vitagee', 02894857, 'Leon', '2322 Riverside Drive', 69011),
  (6, 'Life Map Planners', 35755730, 'Joe', '693 Wood Duck Drive', 36228);

INSERT INTO Goods VALUES
  (1, 'cheeses', 2.5), (2, 'eggs', 1.4), (3, 'milk', 3.4), (4, 'yogurt', 4.0),
  (5, 'butter', 2.0), (6, 'coffee', 3.0), (7, 'tea', 1.5), (8, 'juice', 7.0),
  (9, 'soda', 1.0), (10, 'ice cream', 6.0);

INSERT INTO Supplier VALUES
  (1, 'Listen Up', 73194791,'Jason', '1454 Jessie Street', 45329618),
  (2, 'Prestigabiz', 54976004,'Travis', '3129 Reppert Coal Road', 31735477),
  (3, 'Budget Power', 49164999,'Maxwell', '1523 Dark Hollow Road', 29667962);

INSERT INTO Supplied_Goods VALUES
  (1, 1.5, 1, 1), (2, 1.1, 1, 2), (3, 1.0, 2, 3), (4, 3.2, 3, 1), (5, 3.1, 3, 2),
  (6, 2.0, 4, 3), (7, 1.0, 5, 1), (8, 1.1, 5, 2), (9, 2.4, 6, 3), (10, 2.2, 6, 1),
  (11, 1.1, 7, 2), (12, 5.0, 8, 3), (13, 6.0, 8, 1), (14, 0.2, 9, 2), (15, 0.2, 9, 3),
  (16, 4.0, 10, 1), (17, 4.0, 10, 2);

INSERT INTO Receipt VALUES
  (1, '2017-10-20', true, 1, 4, 4, 100), (2, '2017-10-20', true, 2, 8, 10, 210),
  (3, '2017-10-21', true, 1, 4, 14, 300), (4, '2017-10-25', true, 1, 8, 3, 200),
  (5, '2017-10-27', true, 2, 4, 11, 300), (6, '2017-11-1', true, 2, 8, 11, 400),
  (7, '2017-11-4', false, 1, 4, 12, 500), (8, '2017-11-5', true, 2, 8, 1, 600),
  (9, '2017-11-6', true, 2, 4, 5, 1200), (10, '2017-11-9', false, 1, 8, 8, 210);

INSERT INTO Purchase VALUES
  (1, '2017-10-20', true, 1, 1, 6, 4, 50), (2, '2017-10-20', true, 5, 2, 7, 10, 100),
  (3, '2017-10-21', true, 4, 1, 7, 14, 150), (4, '2017-10-25', true, 6, 1, 2, 3, 50),
  (5, '2017-10-27', true, 2, 2, 2, 4, 40);
