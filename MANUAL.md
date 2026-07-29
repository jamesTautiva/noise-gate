# DjentCut Noise V.1 DEMO - Manual de Usuario

## 1. Introducción

DjentCut Noise es un plugin de noise gate (puerta de ruido) profesional diseñado específicamente para guitarras modernas, especialmente para géneros como djent, metal progresivo, thall, hardcore y deathcore. 

Este plugin ofrece un control preciso sobre el silencio no deseado entre notas y acordes, permitiendo un sonido más tight y definido. Incluye características avanzadas como:

- **Sidechain filtering** (HPF/LPF) para un control más preciso
- **Lookahead** para detección anticipada de transitorios
- **Herramientas de metal**: Pick detector, Palm mute detector, Smart gate
- **Adaptive threshold** que se ajusta dinámicamente
- **Clasificación de ruido** para diferentes tipos de señal
- **Visualización en tiempo real** de waveform y gain reduction

El plugin está optimizado con protecciones contra números denormales y optimización de CPU para un rendimiento eficiente.

## 2. Instalación

### Windows
1. Descarga el archivo `noise-gate.vst3`
2. Copia la carpeta `noise-gate.vst3` a:
   - `C:\Program Files\Common Files\VST3\` (para todos los usuarios)
   - `C:\Users\[TuUsuario]\AppData\Local\Programs\Common\VST3\` (para tu usuario)
3. Reinicia tu DAW (Ableton Live, FL Studio, Reaper, etc.)
4. El plugin aparecerá en la lista de plugins VST3 bajo la categoría "Dynamics" o "Utility"

### macOS
1. Descarga el archivo `noise-gate.vst3`
2. Copia la carpeta `noise-gate.vst3` a:
   - `/Library/Audio/Plug-Ins/VST3/` (para todos los usuarios)
   - `~/Library/Audio/Plug-Ins/VST3/` (para tu usuario)
3. Reinicia tu DAW
4. El plugin aparecerá en la lista de plugins VST3

### Linux
1. Descarga el archivo `noise-gate.vst3`
2. Copia la carpeta `noise-gate.vst3` a:
   - `/usr/lib/vst3/` (para todos los usuarios)
   - `~/.vst3/` (para tu usuario)
3. Asegúrate de que tu DAW soporte VST3 en Linux (Bitwig, Reaper, Ardour con soporte VST3)

## 3. Requisitos

### Sistema
- **Sistema Operativo**: Windows 10+, macOS 10.13+, Linux (distribuciones modernas)
- **Arquitectura**: x86_64 (64-bit)
- **DAW**: Cualquier DAW que soporte plugins VST3

### Hardware
- **CPU**: Procesador moderno de 64-bit (Intel i5+/AMD Ryzen 5+ recomendado)
- **RAM**: Mínimo 4GB, recomendado 8GB+
- **Almacenamiento**: ~50MB de espacio en disco

### Audio
- **Sample Rate**: 44.1kHz - 192kHz
- **Bit Depth**: 16-bit, 24-bit, 32-bit float
- **Canales**: Estéreo (mono soportado internamente)

## 4. Parámetros

### Threshold (Umbral)

**Qué hace:**
El threshold determina el nivel de volumen mínimo que debe superar la señal para que el gate se abra. Si la señal está por debajo de este nivel, el gate se cierra y reduce el volumen.

**Cuándo usarlo:**
- Para eliminar ruido de fondo entre notas
- Para silenciar feedback no deseado
- Para crear efectos staccato más pronunciados
- Para limpiar grabaciones con mucho ruido de ambiente

**Valores recomendados:**
- **Guitarras limpias**: -40dB a -20dB
- **Guitarras distorsionadas**: -30dB a -15dB
- **Bass**: -50dB a -30dB
- **Voces**: -30dB a -20dB
- **Drums (snare)**: -20dB a -10dB

**Explicación técnica:**
El threshold se mide en decibelios (dB) relativos al nivel digital máximo (0dBFS). Un valor más negativo significa que el gate es más sensible (se abre con señales más débiles), mientras que un valor menos negativo hace que el gate sea más estricto.

**Ejemplos:**
- **Threshold -50dB**: El gate se abrirá con señales muy débiles, útil para capturar notas suaves
- **Threshold -20dB**: El gate solo se abrirá con señales moderadamente fuertes
- **Threshold -10dB**: El gate es muy estricto, solo se abre con señales fuertes

**Ejemplos musicales:**
- **Djent riffs**: -25dB a -15dB para un sonido tight y percusivo
- **Thall atmosphere**: -35dB a -25dB para permitir más sustain
- **Deathcore chugs**: -15dB a -10dB para máxima definición
- **Clean arpeggios**: -40dB a -30dB para no cortar notas suaves

### Attack (Ataque)

**Qué hace:**
El attack controla qué tan rápido el gate se abre cuando la señal supera el threshold. Un attack rápido responde inmediatamente, mientras que uno más suave crea un fade-in gradual.

**Cuándo usarlo:**
- **Attack rápido (0.1-5ms)**: Para percusión, palm mutes, riffs rápidos
- **Attack medio (5-20ms)**: Para guitarras limpias, voces
- **Attack lento (20-50ms)**: Para pads, ambientes, efectos especiales

**Valores recomendados:**
- **Guitarras distorsionadas**: 0.1ms - 5ms
- **Guitarras limpias**: 5ms - 15ms
- **Bass**: 1ms - 10ms
- **Voces**: 5ms - 20ms
- **Drums**: 0.01ms - 1ms

**Explicación técnica:**
El attack se mide en milisegundos (ms). Determina el tiempo que tarda el gate en pasar del estado cerrado (gain mínimo) al estado abierto (gain = 1). El coeficiente de attack se calcula como: `attack_coeff = exp(-1 / (sample_rate * attack_time))`

**Ejemplos:**
- **Attack 0.01ms**: Respuesta instantánea, ideal para transitorios muy rápidos
- **Attack 10ms**: Fade-in suave de 10ms
- **Attack 50ms**: Fade-in muy gradual, puede cortar el inicio de notas

**Ejemplos musicales:**
- **Palm mutes**: 0.1ms - 1ms para capturar el attack percusivo
- **Fast alternate picking**: 1ms - 3ms para no perder notas rápidas
- **Clean chords**: 10ms - 20ms para un ataque más natural
- **Ambient swells**: 20ms - 50ms para efectos de fade-in

### Hold (Mantenimiento)

**Qué hace:**
El hold mantiene el gate abierto por un período específico después de que la señal cae por debajo del threshold. Esto evita que el gate cierre prematuramente durante decay natural de notas.

**Cuándo usarlo:**
- Para mantener sustain de notas
- Para evitar "chattering" (apertura/cierre rápido)
- Para permitir decay natural de acordes
- Para efectos de release prolongado

**Valores recomendados:**
- **Guitarras distorsionadas**: 0ms - 50ms
- **Guitarras limpias**: 50ms - 200ms
- **Bass**: 10ms - 100ms
- **Voces**: 50ms - 300ms
- **Pads/Atmospheres**: 200ms - 500ms

**Explicación técnica:**
El hold se mide en milisegundos y representa el tiempo mínimo que el gate permanece abierto después de que la señal cae por debajo del threshold. Esto es especialmente útil para señales con decay natural largo.

**Ejemplos:**
- **Hold 0ms**: Sin hold, el gate cierra inmediatamente cuando la señal cae
- **Hold 100ms**: El gate permanece abierto 100ms después de que la señal cae
- **Hold 500ms**: Hold prolongado para sustain extendido

**Ejemplos musicales:**
- **Staccato riffs**: 0ms - 10ms para máxima separación
- **Power chords**: 50ms - 150ms para sustain natural
- **Clean leads**: 100ms - 300ms para expresividad
- **Atmospheric parts**: 200ms - 500ms para decay completo

### Release (Liberación)

**Qué hace:**
El release controla qué tan rápido el gate se cierra cuando la señal cae por debajo del threshold. Un release rápido crea cortes abruptos, mientras que uno lento crea fades suaves.

**Cuándo usarlo:**
- **Release rápido (5-20ms)**: Para efectos staccato, palm mutes tight
- **Release medio (20-100ms)**: Para balance entre tight y natural
- **Release lento (100-1000ms)**: Para sustain, pads, ambientes

**Valores recomendados:**
- **Guitarras distorsionadas**: 5ms - 50ms
- **Guitarras limpias**: 50ms - 200ms
- **Bass**: 20ms - 100ms
- **Voces**: 50ms - 300ms
- **Pads**: 200ms - 1000ms

**Explicación técnica:**
El release se mide en milisegundos. Determina el tiempo que tarda el envelope en decaer desde su valor actual hacia cero cuando la señal está por debajo del threshold. El coeficiente de release se calcula similar al attack.

**Ejemplos:**
- **Release 5ms**: Cierre muy rápido, efecto staccato pronunciado
- **Release 100ms**: Cierre medio, balance natural
- **Release 500ms**: Cierre lento, sustain prolongado
- **Release 1000ms**: Cierre muy lento, casi imperceptible

**Ejemplos musicales:**
- **Djent chugs**: 5ms - 15ms para máxima tightness
- **Thrash metal**: 10ms - 30ms para agresividad
- **Rock rhythm**: 30ms - 80ms para balance
- **Clean passages**: 100ms - 300ms para naturalidad
- **Ambient sections**: 300ms - 1000ms para decay completo

### Range (Rango)

**Qué hace:**
El range determina cuánto se reduce el volumen cuando el gate está cerrado. Un range de 0dB significa que el gate no reduce nada (bypass), mientras que -80dB es casi silencio total.

**Cuándo usarlo:**
- **Range completo (-80dB)**: Para silenciar completamente el ruido
- **Range parcial (-20dB a -40dB)**: Para reducción sutil de ruido
- **Range ligero (-5dB a -15dB)**: Para gating sutil, efectos de ducking

**Valores recomendados:**
- **Guitarras distorsionadas**: -60dB a -80dB
- **Guitarras limpias**: -40dB a -60dB
- **Bass**: -40dB a -70dB
- **Voces**: -30dB a -50dB
- **Efectos sutiles**: -5dB a -20dB

**Explicación técnica:**
El range se mide en dB y representa la ganancia aplicada cuando el gate está cerrado. Se convierte a ganancia lineal usando: `gain = 10^(range_db / 20)`. Por ejemplo, -6dB ≈ 0.5 de ganancia (mitad de volumen).

**Ejemplos:**
- **Range -80dB**: Silencio casi total
- **Range -40dB**: Reducción significativa pero audible
- **Range -20dB**: Reducción moderada
- **Range -6dB**: Mitad de volumen
- **Range 0dB**: Sin reducción (bypass efectivo)

**Ejemplos musicales:**
- **Metal rhythm**: -60dB a -80dB para silencio total entre notas
- **Clean guitar**: -30dB a -50dB para reducción natural
- **Bass guitar**: -40dB a -60dB para mantener algo de sustain
- **Voces**: -20dB a -40dB para gating sutil
- **Efectos especiales**: -3dB a -10dB para ducking

### Sidechain HPF (High Pass Filter)

**Qué hace:**
El sidechain HPF filtra frecuencias bajas de la señal de sidechain antes de que se use para detectar si el gate debe abrirse. Esto evita que ruido de baja frecuencia (hum, floor noise) active el gate innecesariamente.

**Cuándo usarlo:**
- Cuando hay ruido de baja frecuencia (50Hz/60Hz hum)
- Para que el gate responda solo a frecuencias relevantes
- Para evitar que el gate se abra con vibraciones no deseadas
- Para un control más preciso en guitarras con mucho low-end

**Valores recomendados:**
- **Guitarras distorsionadas**: 100Hz - 400Hz
- **Guitarras limpias**: 80Hz - 200Hz
- **Bass**: 50Hz - 150Hz
- **Voces**: 100Hz - 300Hz
- **Drums**: 150Hz - 500Hz

**Cómo ajustarlo:**
1. Comienza con un valor de 100Hz
2. Aumenta gradualmente hasta que el gate deje de activarse con ruido de baja frecuencia
3. No lo pongas demasiado alto o perderás información importante de la señal
4. Monitorea la visualización de waveform para ver qué frecuencias están activando el gate

**Explicación técnica:**
El HPF es un filtro pasa-altas RC de primer orden con la ecuación: `y[n] = α * (x[n] - x[n-1] + y[n-1])`, donde α depende de la frecuencia de corte y el sample rate.

**Ejemplos:**
- **HPF 20Hz**: Pasa casi todo, filtrado mínimo
- **HPF 100Hz**: Filtra hum y ruido de baja frecuencia
- **HPF 400Hz**: Filtra frecuencias graves, gate responde a mids/highs
- **HPF 800Hz**: Gate responde solo a altas frecuencias

**Casos de uso:**
- **Eliminación de hum**: 50Hz - 100Hz para eliminar hum de alimentación
- **Guitarras con mucho low-end**: 200Hz - 400Hz para que el gate responda al attack
- **Voces con plosives**: 100Hz - 200Hz para reducir sensibilidad a plosives
- **Bass guitar**: 50Hz - 100Hz para mantener respuesta de graves pero filtrar ruido

### Sidechain LPF (Low Pass Filter)

**Qué hace:**
El sidechain LPF filtra frecuencias altas de la señal de sidechain. Esto evita que ruido de alta frecuencia (hiss, sibilance) active el gate innecesariamente.

**Cuándo usarlo:**
- Cuando hay ruido de alta frecuencia (hiss, tape noise)
- Para que el gate responda solo al cuerpo de la señal
- Para evitar que el gate se abra con sibilancia en voces
- Para un control más preciso en señales brillantes

**Valores recomendados:**
- **Guitarras distorsionadas**: 2000Hz - 8000Hz
- **Guitarras limpias**: 5000Hz - 15000Hz
- **Bass**: 500Hz - 2000Hz
- **Voces**: 3000Hz - 10000Hz
- **Drums**: 5000Hz - 20000Hz

**Cómo ajustarlo:**
1. Comienza con un valor alto (15000Hz)
2. Reduce gradualmente hasta que el gate deje de activarse con ruido de alta frecuencia
3. No lo pongas demasiado bajo o perderás attack y transitorios
4. Usa la visualización para ver qué está activando el gate

**Explicación técnica:**
El LPF es un filtro pasa-bajas RC de primer orden con la ecuación: `y[n] = α * x[n] + (1 - α) * y[n-1]`, donde α depende de la frecuencia de corte.

**Ejemplos:**
- **LPF 20000Hz**: Pasa casi todo, filtrado mínimo
- **LPF 8000Hz**: Filtra altas frecuencias, gate responde a mids
- **LPF 2000Hz**: Gate responde solo a bajas/mid frecuencias
- **LPF 500Hz**: Gate responde solo a graves

**Casos de uso:**
- **Eliminación de hiss**: 5000Hz - 10000Hz para eliminar hiss de preamp
- **Guitarras muy brillantes**: 3000Hz - 6000Hz para gate más controlado
- **Voces sibilantes**: 5000Hz - 8000Hz para reducir sensibilidad a sibilance
- **Bass guitar**: 500Hz - 1500Hz para gate que responde al cuerpo del bajo

### Lookahead (Anticipación)

**Qué hace:**
El lookahead introduce un pequeño delay en la señal para permitir que el gate "vea" el audio que viene antes de que llegue, permitiendo una respuesta más precisa a transitorios.

**Cuándo usarlo:**
- Para capturar transitorios muy rápidos
- Para mejorar la precisión en palm mutes
- Para evitar que el gate corte el attack de notas
- Para un control más preciso en señales percusivas

**Valores recomendados:**
- **Guitarras distorsionadas**: 0ms - 5ms
- **Guitarras limpias**: 0ms - 3ms
- **Bass**: 0ms - 2ms
- **Drums**: 0ms - 10ms
- **General**: 0ms - 5ms

**Explicación técnica:**
El lookahead usa un buffer de delay circular. La señal se retrasa por el tiempo de lookahead, pero el detector de envelope opera en la señal original sin delay. Esto permite que el gate se abra antes de que el transitorio llegue al output.

**Ejemplos:**
- **Lookahead 0ms**: Sin anticipación, respuesta en tiempo real
- **Lookahead 2ms**: El gate "ve" 2ms de audio futuro
- **Lookahead 5ms**: Anticipación máxima para transitorios muy rápidos
- **Lookahead 10ms**: Anticipación muy larga, puede causar delay perceptible

**Casos de uso:**
- **Palm mutes tight**: 2ms - 5ms para capturar el attack completo
- **Fast picking**: 1ms - 3ms para no perder notas rápidas
- **Drum transients**: 3ms - 10ms para capturar attack de snare/kick
- **General purpose**: 0ms - 2ms para mejora sutil

## 5. Características Avanzadas

### Pick Detector

**Qué hace:**
Detecta cuando se está haciendo picking rápido y ajusta el comportamiento del gate para adaptarse a este patrón.

**Cuándo usarlo:**
- En secciones de alternate picking rápido
- Para riffs de thrash metal
- En solos con mucho picking
- Para mejorar la respuesta en passages técnicos

**Sensitivity (Sensibilidad):**
Controla qué tan sensible es el detector. Valores más altos detectan picking más suave, valores más bajos solo detectan picking muy agresivo.

### Palm Mute Detector

**Qué hace:**
Detecta palm mutes y ajusta el gate para optimizar la respuesta a esta técnica específica.

**Cuándo usarlo:**
- En riffs con palm mutes
- Para djent y thall
- Para mejorar la tightness de chugs
- Para secciones de metal rítmico

**Threshold (Umbral):**
Determina qué tan fuerte debe ser el palm mute para ser detectado. Valores más altos detectan solo palm mutes muy fuertes.

### Smart Gate

**Qué hace:**
Analiza el envelope de la señal y ajusta dinámicamente el comportamiento del gate para una respuesta más inteligente.

**Cuándo usarlo:**
- Para material con dinámica variable
- Para respuestas más naturales
- Cuando el gate básico es demasiado agresivo
- Para material con sustain variable

### Adaptive Threshold

**Qué hace:**
Ajusta automáticamente el threshold basándose en el nivel de la señal, permitiendo que el gate se adapte a cambios de dinámica.

**Cuándo usarlo:**
- Para material con dinámica muy variable
- Para secciones con cambios de intensidad
- Para evitar que el gate corte passages suaves
- Para respuestas más consistentes

**Speed (Velocidad):**
Controla qué tan rápido se adapta el threshold. Valores más altos adaptan más rápido, valores más bajos son más estables.

### Adaptive Release

**Qué hace:**
Ajusta el tiempo de release dinámicamente basándose en la diferencia entre el nivel actual y el anterior del envelope.

**Cuándo usarlo:**
- Para respuestas más naturales
- Para evitar releases artificiales
- Para material con decay variable
- Para mejorar la musicalidad del gate

### Noise Classification

**Qué hace:**
Clasifica el tipo de ruido o señal presente (silence, noise, signal) y puede ajustar el comportamiento del gate según esta clasificación.

**Cuándo usarlo:**
- Para análisis de señal
- Para ajustes más precisos
- Para entender qué está activando el gate
- Para depuración de problemas

## 6. Presets

El plugin incluye varios presets optimizados para diferentes situaciones:

- **Default**: Configuración balanceada para uso general
- **Modern Metal**: Optimizado para guitarras distorsionadas modernas
- **Djent**: Configuración tight para riffs de djent
- **Thall**: Configuración con más sustain para atmósferas thall
- **Hardcore**: Gate agresivo para hardcore punk
- **Deathcore**: Gate muy tight para deathcore
- **Prog Metal**: Balance entre tight y sustain para metal progresivo
- **Bass**: Optimizado para guitarra baja
- **Clean**: Configuración sutil para guitarras limpias

## 7. Ejemplos de Uso

### Ejemplo 1: Djent Riff Tight

**Objetivo:** Lograr un sonido djent tight y percusivo

**Configuración:**
- Threshold: -18dB
- Attack: 0.5ms
- Hold: 10ms
- Release: 15ms
- Range: -70dB
- Sidechain HPF: 200Hz
- Sidechain LPF: 6000Hz
- Lookahead: 3ms

**Características avanzadas:**
- Pick Detector: ON, Sensitivity: 0.5
- Palm Mute Detector: ON, Threshold: 0.6
- Smart Gate: ON

### Ejemplo 2: Clean Guitar

**Objetivo:** Reducción sutil de ruido en guitarra limpia

**Configuración:**
- Threshold: -35dB
- Attack: 10ms
- Hold: 100ms
- Release: 150ms
- Range: -40dB
- Sidechain HPF: 80Hz
- Sidechain LPF: 12000Hz
- Lookahead: 0ms

**Características avanzadas:**
- Todas las características avanzadas: OFF

### Ejemplo 3: Deathcore Chugs

**Objetivo:** Máxima tightness para chugs agresivos

**Configuración:**
- Threshold: -12dB
- Attack: 0.1ms
- Hold: 5ms
- Release: 8ms
- Range: -80dB
- Sidechain HPF: 300Hz
- Sidechain LPF: 5000Hz
- Lookahead: 5ms

**Características avanzadas:**
- Pick Detector: ON, Sensitivity: 0.7
- Palm Mute Detector: ON, Threshold: 0.8
- Smart Gate: ON
- Adaptive Release: ON

### Ejemplo 4: Bass Guitar

**Objetivo:** Gate controlado para bajo

**Configuración:**
- Threshold: -40dB
- Attack: 2ms
- Hold: 30ms
- Release: 50ms
- Range: -50dB
- Sidechain HPF: 60Hz
- Sidechain LPF: 2000Hz
- Lookahead: 1ms

**Características avanzadas:**
- Smart Gate: ON

## 8. Preguntas Frecuentes

**Q: El gate corta el attack de mis notas. ¿Qué debo hacer?**
A: Reduce el Attack a 0.1ms - 1ms y aumenta el Lookahead a 2ms - 5ms. Esto permite que el gate se abra antes de que llegue el transitorio.

**Q: El gate no se cierra lo suficientemente rápido. ¿Qué ajusto?**
A: Reduce el Release a 5ms - 20ms y reduce el Hold a 0ms - 10ms. Esto creará cortes más abruptos.

**Q: El gate se abre con ruido de fondo. ¿Cómo lo evito?**
A: Aumenta el Threshold (menos negativo) y ajusta el Sidechain HPF a 100Hz - 200Hz para filtrar ruido de baja frecuencia.

**Q: El gate corta el sustain de mis notas. ¿Qué hago?**
A: Aumenta el Hold a 100ms - 300ms y reduce el Release a 50ms - 100ms. También activa el Adaptive Release.

**Q: ¿Cuál es la diferencia entre Range y Threshold?**
A: Threshold determina CUÁNDO se abre el gate, mientras que Range determina CUÁNTO se reduce el volumen cuando está cerrado.

**Q: ¿Debería usar las características avanzadas siempre?**
A: No. Las características avanzadas consumen más CPU y pueden complicar la configuración. Úsalas solo cuando las necesites para problemas específicos.

**Q: El plugin consume mucha CPU. ¿Qué puedo hacer?**
A: Desactiva las características avanzadas que no necesites, reduce el Lookahead a 0ms, y asegúrate de que tu sample rate no sea excesivamente alto.

**Q: ¿Puedo usar este plugin en vivo?**
A: Sí, el plugin está optimizado para uso en vivo. Sin embargo, recomiendo ajustar las configuraciones en un entorno de estudio primero.

**Q: ¿El plugin introduce latencia?**
A: El Lookahead introduce latencia igual al valor configurado. Con Lookahead en 0ms, la latencia es mínima (menos de 1ms).

**Q: ¿Cómo sé si el gate está funcionando correctamente?**
A: Usa la visualización de waveform y gain reduction en tiempo real. El waveform verde (output) debería mostrar las notas claramente, y el gráfico rojo (gain reduction) debería mostrar cuándo el gate está activo.

**Q: ¿Puedo usar este plugin en mono?**
A: Sí, el plugin procesa canales estéreo independientemente, funcionando perfectamente en configuraciones mono.

## 9. Solución de Problebles

### Problema: El gate no se abre

**Posibles causas:**
- Threshold demasiado alto (muy cercano a 0dB)
- Sidechain filters filtrando toda la señal
- Input signal muy débil

**Soluciones:**
1. Reduce el Threshold (más negativo, ej: -50dB)
2. Reduce el Sidechain HPF a 20Hz - 50Hz
3. Aumenta el Sidechain LPF a 20000Hz
4. Verifica que la señal de input tenga suficiente nivel

### Problema: El gate nunca se cierra

**Posibles causas:**
- Threshold demasiado bajo (muy negativo)
- Hold demasiado largo
- Release demasiado lento
- Ruido de fondo constante

**Soluciones:**
1. Aumenta el Threshold (menos negativo, ej: -20dB)
2. Reduce el Hold a 0ms - 10ms
3. Reduce el Release a 5ms - 20ms
4. Aumenta el Sidechain HPF para filtrar ruido

### Problema: Sonido artificial o robótico

**Posibles causas:**
- Attack demasiado rápido
- Release demasiado rápido
- Range demasiado extremo
- Falta de Adaptive Release

**Soluciones:**
1. Aumenta el Attack a 5ms - 15ms
2. Aumenta el Release a 50ms - 150ms
3. Reduce el Range a -30dB - -50dB
4. Activa Adaptive Release

### Problema: Chattering (apertura/cierre rápido)

**Posibles causas:**
- Threshold en el punto crítico de la señal
- Hold muy corto
- Release muy rápido
- Señal con fluctuaciones de nivel

**Soluciones:**
1. Ajusta el Threshold ligeramente más alto o más bajo
2. Aumenta el Hold a 20ms - 50ms
3. Aumenta el Release a 30ms - 80ms
4. Activa Smart Gate

### Problema: El gate corta transitorios

**Posibles causas:**
- Attack demasiado lento
- Lookahead desactivado
- Sidechain filters filtrando transitorios

**Soluciones:**
1. Reduce el Attack a 0.1ms - 1ms
2. Activa Lookahead a 2ms - 5ms
3. Reduce el Sidechain HPF y aumenta el Sidechain LPF

### Problema: CPU alta

**Posibles causas:**
- Muchas características avanzadas activadas
- Sample rate muy alto
- Lookahead alto
- Múltiples instancias del plugin

**Soluciones:**
1. Desactiva características avanzadas no necesarias
2. Reduce sample rate a 44.1kHz o 48kHz
3. Reduce Lookahead a 0ms - 2ms
4. Usa una sola instancia en el bus master si es posible

### Problema: No hay sonido en el output

**Posibles causas:**
- Range en 0dB (bypass efectivo)
- Gate permanentemente cerrado
- Problema de routing en el DAW
- Plugin en bypass

**Soluciones:**
1. Verifica que el Range no sea 0dB
2. Reduce el Threshold significativamente
3. Verifica routing en el DAW
4. Asegúrate de que el plugin no esté en bypass

## 10. Créditos

**Desarrollo:**
- Diseño y desarrollo: [Tu Nombre/Equipo]
- Arquitectura de audio: Basado en nih-plug framework
- Optimización SIMD: Protección denormal y optimización de CPU

**Inspiración:**
- Diseñado inspirado en las necesidades de guitarristas de metal moderno
- Influencias de gates profesionales en la industria
- Optimizado para estilos: Djent, Thall, Deathcore, Prog Metal

**Agradecimientos:**
- nih-plug framework por la infraestructura de plugin
- Comunidad de audio DSP por recursos y conocimiento
- Beta testers por feedback valioso

## 11. Licencia

**Versión:** V.1 DEMO

**Estado:** Versión de demostración

**Limitaciones de la DEMO:**
- Esta versión es una demostración de funcionalidades
- Puede tener limitaciones de tiempo o características
- Versión completa disponible en [URL cuando esté disponible]

**Uso permitido:**
- Uso personal y educativo
- Testing y evaluación
- No comercial en versión DEMO

**Licencia completa:**
- La versión completa estará sujeta a licencia comercial
- Contacto para licencias: [email cuando esté disponible]
- Términos y condiciones completos en [URL cuando esté disponible]

**Soporte:**
- Reporte de bugs: [URL cuando esté disponible]
- Documentación actualizada: [URL cuando esté disponible]
- Comunidad: [URL cuando esté disponible]

---

**Versión del manual:** 1.0  
**Fecha:** Julio 2026  
**Plugin version:** 0.1.0 DEMO  
**Compatible con:** VST3 (Windows, macOS, Linux)
