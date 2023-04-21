drop schema if exists resource;
create schema resource;

use resource;

drop table if exists basic_grade;
create table basic_grade
(
    code    varchar(3) primary key,
    name_en varchar(128) not null
);
insert into basic_grade
values ('441', 'Staff/Assistant 1')
     , ('442', 'Staff/Assistant 2')
     , ('421', 'Senior 1')
     , ('422', 'Senior 2')
     , ('321', 'Manager 1')
     , ('322', 'Manager 2')
     , ('211', 'Senior Manager 1')
     , ('212', 'Senior Manager 2')
     , ('111', 'Partner/Principal 1')
     , ('112', 'Partner/Principal 2')
     , ('511', 'Intern (CS) 1');

drop table if exists basic_legal_entity;
create table basic_legal_entity
(
    code    varchar(5) primary key,
    name_en varchar(128) not null
);
insert into basic_legal_entity
values ('CNL01', 'Smart Fitness Ltd')
     , ('CNL02', 'Garden Glow Ltd')
     , ('CNL03', 'Vortex Solar LLP')
     , ('CNL04', 'Ready Continental LLP')
     , ('CNL05', 'Coal Kings Ltd')
     , ('HKL01', 'Lambent Illumination Ltd')
     , ('HKL02', 'Sanguine Skincare Ltd')
     , ('HKL03', 'Moxie Marketing Ltd');

drop table if exists basic_staff;
create table basic_staff
(
    uuid              varchar(36) primary key,
    name              varchar(255) not null,
    grade_code        varchar(3)   not null,
    legal_entity_code varchar(5)   not null
);
insert into basic_staff
values ('eb24e23d-c827-4add-8b93-be24b850f7d0', 'Þórvaldr Ratna', '441', 'CNL01')
     , ('b87e013c-b275-41b7-ab92-8aba942d7866', 'Romy Dosifey', '441', 'CNL01')
     , ('262345c3-9837-4480-be7c-65e92ae2d0a9', 'Winfrith Concetto', '441', 'CNL01')
     , ('84030296-2a53-4f75-9fa8-31a199a2ad20', 'Baraz Wulfstan', '441', 'CNL01')
     , ('76b29035-15ff-4e58-97fd-c39bbef8b4b3', 'Henryk Prem', '441', 'CNL01');