# Changelog

All notable changes to `ShellOrd` will be documented in this file

----

## SHELLORD v23.5.22

- **Mejoras:**
  - API para la creación de extensiones. Ahora es posible añadir metadatos e iconos.
  - URL internas con Bech32 cifradas.

- **Bugs:**
  - Arreglo al cargar mas de 50MB en ficheros MAR.

- **Cambios:**
  - Se elimina el modo old. Se ha eliminado por seguridad, debido a que todas las claves ahora son aleatorias por seguridad.
  
- **Seguridad**
  - Las claves para encriptar los secretos (variables de entorno cifradas para usar SIAM, la IA que usa ChatGPT,BLOOM,Bard & Bing Chat todo en uno), y ficheros son generadas de forma aleatoria usando AES256 GCM HKDF 1MB. El tipo de cifrado es personalizable. Se usan N permutaciones que tambien es configurable para mayor protección. **ATENCION:** Exporta la clave y guardala en un lugar seguro!! Si pierdes o generas una nueva clave perderas el acceso a todos los datos.

## Autor

[Creado por Miguel J. Carmona (MIBALTOALEX)][1]

[1]: <https://me.mibaltoalex.com/>  "Miguel J. Carmona (MIBALTOALEX)"
