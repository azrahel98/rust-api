CREATE DEFINER = `` @`` PROCEDURE `insertar_detalle_doc` (
	IN doc INT,
	IN dni VARCHAR ( 8 ),
	IN fecha VARCHAR ( 10 ),
	IN asunto VARCHAR ( 255 ),
	IN referencia VARCHAR ( 255 ),
	IN descripcion VARCHAR ( 255 ),
	IN inicio VARCHAR ( 10 ),
	IN fin VARCHAR ( 10 ),
	IN secreto VARCHAR ( 10 )
	) BEGIN
	DECLARE
		error_code INT;
	DECLARE
		CONTINUE HANDLER FOR 1062 BEGIN
			
			SET error_code = 1062;
		
	END;
	DECLARE
		CONTINUE HANDLER FOR 1452 BEGIN
			
			SET error_code = 1452;
		
	END;
	START TRANSACTION;
	INSERT INTO detalledoc ( doc, dni, fecha, asunto, referencia, descripcion, inicio, fin )
	VALUES
		(
			doc,
			dni,
			fecha,
			AES_ENCRYPT( asunto, secreto ),
			AES_ENCRYPT( referencia, secreto ),
			AES_ENCRYPT( descripcion, secreto ),
			inicio,
			fin 
		);
	IF
		error_code = 1062 THEN
		SELECT
			'Error: Ya existe un detalle de documento con este ID' AS Mensaje;
		
		ELSEIF error_code = 1452 THEN
		SELECT
			'Error: La clave foránea no existe en la tabla relacionada' AS Mensaje;
		ELSE 

		SELECT
			LAST_INSERT_ID() AS Resultado;
		
	END IF;

END