CREATE TABLE `Usuario` (
  `id` int PRIMARY KEY AUTO_INCREMENT,
  `nickname` varchar(255) not null,
  `password` varchar(255) not null,
  `nombre` varchar(255) not null,
  `created_at` timestamp default CURRENT_TIMESTAMP
);

CREATE TABLE `Datos_Generales` (
  `dni` varchar(8) PRIMARY KEY,
  `nombre` varchar(255) not null,
  `apellidos` varchar(255) not null,
  `sexo` enum('Y','N') default 'Y',
  `nacimiento` date ,
  `direccion` varchar(255),
  `telf` varchar(255),
  `email` varchar(255),
  `discapacitado` enum('Y','N'),
  `fotosheck` enum('Y','N'),
  `pension` varchar(255),
  `CUSSP` varchar(255)
);

CREATE TABLE `Contrato` (
  `id` int PRIMARY KEY AUTO_INCREMENT,
  `dni` varchar(8) not null,
  `numero` varchar(255) ,
  `ingreso` date not null,
  `renuncia` date,
  `convocatoria` int,
  `convocatoria_s` varchar(255),
  `area` int not null,
  `funcion` int not null,
  `activo` enum('Y','N'),
  `cargo` int not null,
  `regimen` int not null
);

CREATE TABLE `Area` (
  `id` int PRIMARY KEY AUTO_INCREMENT,
  `nombre` varchar(255) UNIQUE not null
);

CREATE TABLE `Cargo` (
  `id` int PRIMARY KEY AUTO_INCREMENT,
  `nombre` varchar(255) UNIQUE not null
);

CREATE TABLE `Funciones_Contratos` (
  `id` int PRIMARY KEY AUTO_INCREMENT,
  `f1` varchar(255),
  `f2` varchar(255),
  `f3` varchar(255),
  `f4` varchar(255),
  `f5` varchar(255),
  `f6` varchar(255),
  `f7` varchar(255),
  `f8` varchar(255),
  `f9` varchar(255),
  `f10` varchar(255)
);

CREATE TABLE `Regimen` (
  `id` int PRIMARY KEY AUTO_INCREMENT,
  `nombre` varchar(255) UNIQUE not null
);

CREATE TABLE `Documentos` (
  `docid` int PRIMARY KEY AUTO_INCREMENT,
  `fecha` date not null,
  `tipo` int not null,
  `nombre` varchar(255) UNIQUE not null,
  `created_at` timestamp default CURRENT_TIMESTAMP,
  `create_by` int
);

CREATE TABLE `TipoDoc` (
  `id` int PRIMARY KEY AUTO_INCREMENT,
  `nombre` varchar(255) UNIQUE not null
);

CREATE TABLE `DetalleDoc` (
  `id` int PRIMARY KEY AUTO_INCREMENT,
  `doc` int not null,
  `dni` varchar(8) not null,
  `descripcion` varchar(255) not null,
  `fecha` date not null,
  `referencia` varchar(255) not null,
  `asunto` enum('HORASEXTRAS','ONOMASTICO','OMISION','LICENCIA','SANCION','DM','RENUNCIA','DF','AC','JUSTIFICADO','DFXHEL','OTROS') not null,
  `inicio` date not null,
  `fin` date not null,
  `active` enum('Y','N') default 'Y',
  `created_at` timestamp default CURRENT_TIMESTAMP,
  `create_by` int
);


CREATE TABLE `Convocatoria` (
  `id` int PRIMARY KEY AUTO_INCREMENT,
  `numero` int not null,
  `year` int not null,
  `area` int not null,
  `cargo` int not null,
  `desierto` enum('Y','N') default 'N',
  `sueldo` int,
  `funcion` int
);

CREATE TABLE `Postulantes` (
  `id` int PRIMARY KEY AUTO_INCREMENT,
  `dni` varchar(8) ,
  `nombre` varchar(255),
  `convocatoria` int,
  `apto` enum('Y','N') default 'N' ,
  `ganador` enum('Y','N') default 'N'
);

ALTER TABLE `Contrato` ADD FOREIGN KEY (`dni`) REFERENCES `Datos_Generales` (`dni`);

ALTER TABLE `Contrato` ADD FOREIGN KEY (`funcion`) REFERENCES `Funciones_Contratos` (`id`);

ALTER TABLE `Contrato` ADD FOREIGN KEY (`regimen`) REFERENCES `Regimen` (`id`);

ALTER TABLE `Contrato` ADD FOREIGN KEY (`area`) REFERENCES `Area` (`id`);

ALTER TABLE `Contrato` ADD FOREIGN KEY (`cargo`) REFERENCES `Cargo` (`id`);

ALTER TABLE `Documentos` ADD FOREIGN KEY (`tipo`) REFERENCES `TipoDoc` (`id`);

ALTER TABLE `DetalleDoc` ADD FOREIGN KEY (`dni`) REFERENCES `Datos_Generales` (`dni`);

ALTER TABLE `DetalleDoc` ADD FOREIGN KEY (`doc`) REFERENCES `Documentos` (`docid`);

ALTER TABLE `DetalleDoc` ADD FOREIGN KEY (`create_by`) REFERENCES `Usuario` (`id`);

ALTER TABLE `Documentos` ADD FOREIGN KEY (`create_by`) REFERENCES `Usuario` (`id`);

ALTER TABLE `Postulantes` ADD FOREIGN KEY (`convocatoria`) REFERENCES `Convocatoria` (`id`);

ALTER TABLE `Convocatoria` ADD FOREIGN KEY (`funcion`) REFERENCES `Funciones_Contratos` (`id`);

ALTER TABLE `Contrato` ADD FOREIGN KEY (`convocatoria`) REFERENCES `Convocatoria` (`id`);

