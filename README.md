<center>
    <h1 align="center">🚀 Projectile</h1>
    <h4 align="center"><b>Projectile</b> es una simple aplicación para gestionar las bases de proyecto, escrita completamente en Rust.</h4>
</center>

## Como funciona

Para empezar a usar **projectile**, primero descarga la última versión en la sección de [releases](https://github.com/holasoyender/Projectile/releases) del repositorio de **GitHub**, si tienes una versión antigua el programa detectará automáticamente la nueva versión y te preguntara para actualizarse automáticamente.

La primera vez que abras el programa te saltará un mensaje de bienvenida, de este lo más importante es saber que el archivo de configuración se encuentra en `C:\Users\<user>\AppData\Roaming\Projectile\settings.yaml`

Despues de esto el programa se instalará en la misma carpeta que el archivo de configuración.

<center>
<img align="center" src="https://projectile-api.kirobot.cc/img/banner.png" height="343" width="786"/>
</center>

### Instalar

Una vez que el programa se haya abierto y copiado, debes de añadir dicha carpeta ( `C:\Users\<user>\AppData\Roaming\Projectile`) a la variable **path** de tu sistema, si no sabes como puedes mirar [esta guía](https://github.com/holasoyender/Projectile/blob/main/docs/path.md)

Con esto podras acceder al comando `projectile` desde la consola en cualquier carpeta de tu sistema.

## Guía de uso

Una vez establecida la carpeta en el **path**, entra a la carpeta en la que quieres iniciar un proyecto y escribe el comando `projectile`, esto abrirá el programa.

Una vez iniciado el programa este buscará una actualización, y en caso de encontrarla te pedirá permiso para instalarla, de lo contrario aparecerá en la consola una lista de todas las base de proyectos que has establecido en la carpeta de proyectos.

Para seleccionar un proyecto tan solo escribe su respectivo número en la consola, si este no tiene un archivo `proyect.yml` o no se han encontrado variables la copia de archivos empezará automáticamente.

En caso de existir variables, el programa te pedirá rellenar todas las variables una a una, si no se especifica ningún valor el valor pasara a ser `None`

Cuando todos los archivos hayan terminado de copiarse, comenzará la ejecución de todos los scripts del archivo de configuración del proyecto en el caso de que se han especificado.

Despues de esto el programa se cerrará automaticamente pasados 5 segundos.

## Crear un proyecto

Para crear una nueva base de proyecto, debes de crear una carpeta en el directorio de proyectos, por defecto esa carpeta se encuentra en `C:\Users\<user>\AppData\Roaming\Projectile\projects`, pero puedes cambiarla en cualquier momento desde el archivo de configuración. El nombre de la carpeta será el nombre del proyecto.

Es recomendable crear dentro del proyecto un archivo `proyect.yml`, que será leído por el programa para leer las variables y los scripts, su sintaxis es la siguiente:
```yml
name: "Nombre del proyecto"
description: "Descripción del proyecto"
vars:
 - variable_uno: "Nombre de la variable 1"
 - variable_dos: "Nombre de la variable 2"
 ...

scripts:
 - "script uno"
 - "script dos"
 ...
```

En cualquier archivo del proyecto se puede añadir la sintaxis `{{nombre_variable}}`, y será remplazada con el valor que se haya establecido para esa variable al crear el proyecto con el comando `projectile`

### Ejemplo

Si creo un proyecto, y en su `proyect.yml` especifico esta variable:
```yml
vars:
 - nombre: "Nombre del propietario del proyecto"
 ```
Cuando inicie el programa  y seleccione el proyecto, me pedirá un valor para la variable "Nombre del propietario del proyecto", en mi caso pondré **Guille**

Ahora mi archivo `index.js` pasará de tener este contenido:
```js
console.log("Hola, {{nombre}}");
```
A este
```js
console.log("Hola, Guille"); 
```
Se puede encontrar un ejemplo completo en la [carpeta de ejemplos](https://github.com/holasoyender/Projectile/tree/main/examples)

## Scripts

Se pueden especificar scripts en el archivo de configuración del proyecto, cuando todos los archivos se copien los scripts .

Los scripts se ejecutarán en orden descendente, y su sintaxis es la siguiente:
```yml
scripts:
 - "script uno"
 - "script dos"
 ...
 ```
 En este ejemplo, primero se ejecutará el "script uno" y luego el "script dos"

 ## Compilar

 El ejecutable principal está escrito por completo en **rust**, para ser compilado solo hay que ejecutar este comando en el directorio del proyecto:
 ```shell
 $ cargo build --release
 ```
 Este programa solo puede ser compilado y ejecutado en **Windows 10/11**

 ## Licencia

 Projectile está bajo la [Apache License, Version 2.0.](https://github.com/holasoyender/Projectile/blob/main/LICENSE)
