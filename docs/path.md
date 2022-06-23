# Como instalar **projectile** en el path de tu ordenador

Al instalar **projectile** en el path de tu sistema, podrás acceder al comando `projectile` desde la linea de comandos en cualquier carpeta, si este paso se omite se tendrá que copiar manualmente el archivo `projectile.exe` a la carpeta deseada.

## Paso 1: Localizar la carpeta de instalación

**Projectile** se instala en la carpeta `C:\Users\<user>\AppData\Roaming\Projectile`, el primer paso será ir a esa carpeta y copiar la ruta, sustituyendo `<user>` por el usuario del sistema.

## Paso 2: Abrir las variables del entorno

Ahora en el buscador de windows busca "variables de entorno"

<img src="https://projectile-api.kirobot.cc/img/path1.png" height="315" width="782">

Se te abrirá una pestaña, en la que debes hacer click a "Variables de entorno".

<img src="https://projectile-api.kirobot.cc/img/path2.png" height="493" width="415">

## Paso 3: Localiza la variable **"path"**

Una vez dentro del menú, debes localizar la variable **"Path"** dentro de la sección **"Variables del sistema"**, una vez encontrada hazla click y luego click en Editar.

<img src="https://projectile-api.kirobot.cc/img/path3.png" height="588" width="618">

## Paso 4: Añadir la carpeta

A continuación, haz click en el botón "Nuevo" y pega la carpeta del **paso 1**, luego haz click en Aceptar

<img src="https://projectile-api.kirobot.cc/img/path4.png" height="503" width="527">

## Paso 5: Comprobar

Si lo has hecho todo bien, ahora podrás abrir una terminal y escribir el comando `projectile`!
