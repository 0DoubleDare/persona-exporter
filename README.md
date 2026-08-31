# Persona Exporter
[![Build and test](https://github.com/0DoubleDare/persona-exporter/actions/workflows/main.yml/badge.svg)](https://github.com/0DoubleDare/persona-exporter/actions/workflows/main.yml)
![GitHub repo size](https://img.shields.io/github/repo-size/0DoubleDare/persona-exporter)
![GitHub License](https://img.shields.io/github/license/0DoubleDare/persona-exporter)

---

## Описание
Легковесный экспортер метрик написанный на Rust работающий по push-модели.
- [Работа с экспортером](#работа-с-экспортером)
  - [Запуск сервиса systemd](#запуск-сервиса)
  - [Первичная настройка](#первичная-настройка)
  - [Собираемые метрики](#собираемые-метрики)
    - [Система](#система-system)
    - [Оперативная память RAM](#оперативная-память-memory)
    - [Диск](#диск-disk)
    - [Сеть](#сеть-network)
    - [Процессор](#процессор-cpu)
    - [Датчики](#датчики-и-компоненты-components)
    - [UNIX-Время](#время-time)
  - [Файл конфигурации](#конфигурация)
- [Планы развития](#планы-развития)

---

## Работа с экспортером
### Запуск сервиса
```bash
# Запуск программы 
systemctl start persona-exporter.service
```
```bash
# Проверка состояния сервиса
systemctl status persona-exporter.service
```
```bash
# Открыть логи 
journalctl -u persona-exporter.service -f
```

---

### Первичная настройка
Основной файл конфигурации **config.yaml** по умолчанию хранится в ```/etc/persona-exporter/config.toml```.
Рабочую директорию можно изменить через переменную окружения ```PERSONA_EXPORTER_CONFIG_DIR```.
К примеру 
```bash
export PERSONA_EXPORTER_CONFIG_DIR="~/.config/persona-exporter/" 
```

---

### Собираемые метрики
Объемы (к примеру, объяем оперативной памяти, дискового пространства) предоставляются в сырых **байтах**.

---

### Система (`system`)
Объект содержит общую информацию об операционной системе и её текущем состоянии.
* **Имя** (`name`) — название операционной системы или дистрибутива.
* **Версия ядра** (`kernel_version`) — короткая версия системного ядра.
* **Полная версия системы** (`kernel_long_version`) — название платформы вместе с версией ядра.
* **Имя дистрибутива** (`distribution_id`) — уникальный идентификатор конкретного дистрибутива в нижнем регистре.
* **Родители дистрибутива** (`distribution_id_like`) — список родительских систем, от которых произошел данный дистрибутив.
* **Время запуска** (`boot_time`) — время включения устройства в формате UNIX-время (в секундах).
* **Время работы** (`uptime`) — время непрерывной работы устройства в секундах с момента его включения.
* **Архитектура процессора** (`cpu_arch`) — архитектура процессора.
* **Версия ОС / Дистрибутива** (`os_version`) — номер версии операционной системы.
* **Имя хоста** (`host_name`) — сетевое имя компьютера (имя машины).
* **Средняя загрузка** (`load_average`) — объект со средней нагрузкой на систему за разные промежутки времени:
    * `one` — за последнюю 1 минуту.
    * `five` — за последние 5 минут.
    * `fifteen` — за последние 15 минут.

---

### Список процессов (`process_list`)
Содержит информацию о процессах в системе и состоит из двух частей.
* **Метрики экспортера** (`exporter_metrics`) — данные о процессе самого сборщика метрик (структура полей совпадает с обычным процессом ниже).
* **Список процессов** (`process_list`) — массив объектов, где каждый процесс имеет следующие поля:
    * **Имя** (`name`) — название исполняемого файла или процесса.
    * **Статус** (`status`) — текущее состояние процесса (например, `Run`, `Sleep`, `Idle`).
    * **Использование диска** (`disk_usage`) — статистика чтения и записи на диск:
        * `read_bytes` — байт прочитано за последний промежуток обновления.
        * `written_bytes` — байт записано за последний промежуток обновления.
        * `total_read_bytes` — всего прочитано байт за всё время жизни процесса.
        * `total_written_bytes` — всего записано байт за всё время жизни процесса.
    * **Идентификатор программы** (`program_id`) — уникальный номер процесса в системе (PID).
    * **Использование CPU** (`cpu_usage`) — процент загрузки процессора данным процессом.
    * **Использование памяти** (`memory_usage`) — объем занимаемой физической оперативной памяти (в байтах).
    * **Виртуальная память** (`virtual_memory`) — объем выделенной процессу виртуальной памяти (в байтах).
    * **Время работы** (`run_time`) — время жизни процесса в секундах.
    * **Время старта** (`start_time`) — время запуска процесса в формате UNIX-время (в секундах).
    * **ID пользователя** (`user_id`) — идентификатор пользователя (UID), запустившего процесс.
    * **ID группы** (`group_id`) — идентификатор группы (GID), к которой относится процесс.

---

### Оперативная память (`memory`)
Показывает состояние физической памяти (RAM) и пространства подкачки (Swap) в байтах.
* **Всего памяти** (`total_memory`) — общий объем установленной оперативной памяти.
* **Использовано памяти** (`used_memory`) — объем занятой оперативной памяти.
* **Свободно памяти** (`free_memory`) — объем полностью неиспользуемой памяти.
* **Доступно памяти** (`available_memory`) — объем памяти, который может быть выделен процессам без ухода в Swap (включая кэш).
* **Всего файла подкачки** (`total_swap`) — общий объем пространства Swap.
* **Использовано файла подкачки** (`used_swap`) — объем занятого пространства Swap.
* **Свободно файла подкачки** (`free_swap`) — объем доступного пространства Swap.

---

### Диск (`disk`)
Информация о главном накопителе системы.
* **Имя** (`name`) — системный путь к разделу или диску (например, `/dev/nvme0n1p2`).
* **Файловая система** (`file_system`) — тип файловой системы (например, `ext4`, `ntfs`).
* **Тип** (`kind`) — тип физического накопителя (например, `SSD`, `HDD`).
* **Всего места** (`total_space`) — общий объем диска в байтах.
* **Доступное место** (`available_space`) — объем свободного места на диске в байтах.

---

### Сеть (`network`)
Статистика сетевой активности основного интерфейса.
* **Имя интерфейса** (`interface_name`) — системное имя сетевого адаптера (например, `wlp0s20f3`).
* **Всего принято байт** (`total_rx_bytes`) — общий объем полученных данных (Download) за всё время.
* **Всего принято пакетов** (`total_rx_packets`) — количество успешно полученных сетевых пакетов.
* **Ошибки приема** (`total_rx_errors`) — количество ошибок при получении данных.
* **Всего отправлено байт** (`total_tx_bytes`) — общий объем отправленных данных (Upload) за всё время.
* **Всего отправлено пакетов** (`total_tx_packets`) — количество успешно отправленных сетевых пакетов.
* **Ошибки отправки** (`total_tx_errors`) — количество ошибок при отправке данных.

---

### Процессор (`cpu`)
Общие технические показатели центрального процессора.
* **Использование CPU** (`cpu_usage`) — суммарная текущая нагрузка на процессор в процентах.
* **Потоки** (`threads`) — количество логических ядер (потоков) процессора.
* **Количество физических ядер** (`physical_core_count`) — количество реальных физических ядер.

---

### Датчики и компоненты (`components`)
Информация с датчиков температуры вашей материнской платы, процессора и дисков.
* **Количество** (`count`) — общее число обнаруженных датчиков в системе.
* **Флаг пустоты** (`is_empty`) — показывает, пуст ли список датчиков (`true`/`false`).
* **Список компонентов** (`components`) — массив объектов датчиков со следующими полями:
    * **ID** (`id`) — уникальный системный идентификатор датчика.
    * **Имя** (`name`) — понятное название компонента или зоны замера (ядро CPU, датчик NVMe и т.д.).
    * **Температура** (`temp`) — текущая температура в градусах Цельсия.
    * **Критическая температура** (`critical_temp`) — порог температуры, при котором система может аварийно отключиться.
    * **Максимальная температура** (`max_temp`) — максимальное зафиксированное значение температуры.

---

### Время (`time`)
* **Текущее время** (`time`) — точное системное время в наносекундах (Unix timestamp в наносекундах), полученное на момент сбора метрик.
 `Время`

---

#### Как выглядят метрики в JSON формате (пример)
```json
{
  "system": {
    "name": "NixOS",
    "kernel_version": "7.0.10-zen1",
    "kernel_long_version": "Linux 7.0.10-zen1",
    "distribution_id": "nixos",
    "distribution_id_like": [],
    "boot_time": 1786947931,
    "uptime": 85991,
    "cpu_arch": "x86_64",
    "os_version": "26.05",
    "host_name": "nikita",
    "load_average": {
      "one": 1.1,
      "five": 1.31,
      "fifteen": 1.35
    }
  },
  "process_list": {
    "exporter_metrics": {
      "name": "persona-exporte",
      "status": "Run",
      "disk_usage": {
        "read_bytes": 0,
        "written_bytes": 139284480,
        "total_read_bytes": 0,
        "total_written_bytes": 139284480
      },
      "program_id": "353498",
      "cpu_usage": 0,
      "memory_usage": 67641344,
      "virtual_memory": 1651945472,
      "run_time": 1,
      "start_time": 1787033921,
      "user_id": "1000",
      "group_id": "100"
    },
    "process_list": [
      {
        "name": "WrGlyph~terizer",
        "status": "Sleep",
        "disk_usage": {
          "read_bytes": 1531904,
          "written_bytes": 0,
          "total_read_bytes": 1531904,
          "total_written_bytes": 0
        },
        "program_id": "6726",
        "cpu_usage": 0,
        "memory_usage": 805584896,
        "virtual_memory": 13410684928,
        "run_time": 85932,
        "start_time": 1786947990,
        "user_id": "1000",
        "group_id": "100"
      },
      {
        "name": "WRWorkerLP#2",
        "status": "Sleep",
        "disk_usage": {
          "read_bytes": 0,
          "written_bytes": 0,
          "total_read_bytes": 0,
          "total_written_bytes": 0
        },
        "program_id": "6720",
        "cpu_usage": 0,
        "memory_usage": 805584896,
        "virtual_memory": 13410684928,
        "run_time": 85932,
        "start_time": 1786947990,
        "user_id": "1000",
        "group_id": "100"
      },
      {
        "name": "gdbus",
        "status": "Sleep",
        "disk_usage": {
          "read_bytes": 0,
          "written_bytes": 0,
          "total_read_bytes": 0,
          "total_written_bytes": 0
        },
        "program_id": "1529",
        "cpu_usage": 0,
        "memory_usage": 10133504,
        "virtual_memory": 398196736,
        "run_time": 85985,
        "start_time": 1786947937,
        "user_id": "0",
        "group_id": "0"
      },
      {
        "name": "Worker Launcher",
        "status": "Sleep",
        "disk_usage": {
          "read_bytes": 8192,
          "written_bytes": 0,
          "total_read_bytes": 8192,
          "total_written_bytes": 0
        },
        "program_id": "6942",
        "cpu_usage": 0,
        "memory_usage": 111058944,
        "virtual_memory": 2768277504,
        "run_time": 85930,
        "start_time": 1786947992,
        "user_id": "1000",
        "group_id": "100"
      },
      {
        "name": "Isolated Web Co",
        "status": "Sleep",
        "disk_usage": {
          "read_bytes": 11702272,
          "written_bytes": 0,
          "total_read_bytes": 11702272,
          "total_written_bytes": 0
        },
        "program_id": "180689",
        "cpu_usage": 0,
        "memory_usage": 858603520,
        "virtual_memory": 3765645312,
        "run_time": 59960,
        "start_time": 1786973962,
        "user_id": "1000",
        "group_id": "100"
      }
    ]
  },
  "memory": {
    "total_memory": 16543600640,
    "used_memory": 9833271296,
    "free_memory": 503083008,
    "available_memory": 6710329344,
    "total_swap": 25451552768,
    "used_swap": 917835776,
    "free_swap": 24533716992
  },
  "disk": {
    "name": "/dev/nvme0n1p2",
    "file_system": "ext4",
    "kind": "SSD",
    "total_space": 501889327104,
    "available_space": 86368546816
  },
  "network": {
    "interface_name": "wlp0s20f3",
    "total_rx_bytes": 7457014974,
    "total_rx_packets": 5190777,
    "total_rx_errors": 0,
    "total_tx_bytes": 284549525,
    "total_tx_packets": 2292098,
    "total_tx_errors": 0
  },
  "cpu": {
    "cpu_usage": 14.814815,
    "threads": 8,
    "physical_core_count": 4
  },
  "components": {
    "count": 8,
    "is_empty": false,
    "components": [
      {
        "id": "hwmon4_1",
        "name": "coretemp Package id 0",
        "temp": 84,
        "critical_temp": 100,
        "max_temp": 84
      },
      {
        "id": "hwmon4_2",
        "name": "coretemp Core 0",
        "temp": 84,
        "critical_temp": 100,
        "max_temp": 84
      },
      {
        "id": "hwmon4_5",
        "name": "coretemp Core 3",
        "temp": 54,
        "critical_temp": 100,
        "max_temp": 54
      },
      {
        "id": "hwmon4_4",
        "name": "coretemp Core 2",
        "temp": 64,
        "critical_temp": 100,
        "max_temp": 64
      },
      {
        "id": "hwmon4_3",
        "name": "coretemp Core 1",
        "temp": 60,
        "critical_temp": 100,
        "max_temp": 60
      },
      {
        "id": "hwmon0_1",
        "name": "nvme Composite 511BS0512HB",
        "temp": 27.85,
        "critical_temp": 79.85,
        "max_temp": 27.85
      },
      {
        "id": "hwmon5_1",
        "name": "iwlwifi_1 temp1",
        "temp": 44,
        "critical_temp": 0,
        "max_temp": 44
      },
      {
        "id": "hwmon3_1",
        "name": "acpitz temp1",
        "temp": 74,
        "critical_temp": 0,
        "max_temp": 74
      }
    ]
  },
  "time": 1787033923350927711
}
```

---

### Конфигурация
Теперь подробнее про конфигурацию, как упоминалось ранее по умолчанию файл конфигурации
находится в `/etc/persona-exporter/config.yaml` и можно изменить директорию с помощью
переменной окружения `PERSONA_EXPORTER_CONFIG_DIR`.
Ниже будут описаны все поля конфигурации. Для наглядности мы опишем 
вполне работающую конфигурацию для Influx DB v2
```yaml
agent:
  # Интервал отправки метрик
  send_interval: 10
  # Формат данных которые будут отправляться на сервер.
  # "json" / "line_protocol"
  data_type: "line_protocol"

server:
  push:
    # Целевой сервер (url)
    url: "http://localhost:8086/api/v2/write"
    # Заголовки которые будут переданы в тело запроса, нужно к примеру
    # для токенов авторизации
    http_headers:
      - key: "Authorization"
        value: "Token ${INFLUX_DB_TOKEN}"
      - key: "Content-Type"
        value: "text-plain; charset=utf-8"
    # Дополнительно устанавливаем URL переменные которые требует Influx DB v2
    # Они добавятся в конце ващего URL
    url_params:
      # Указываем нашу организацию
      - key: "org"
        value: "my-great-company"
      # Указываем название БД
      - key: "bucket"
        value: "org-server-metrics"
      # Указываем в каком формате UNIX-время, ns - наносекунды
      - key: "precision"
        value: "ns"

# Далее по необходиомости описываем конкретные категории метрик
metrics:
  cpu:
    enable: true

  system:
    enable: true

  processes:
    enable: true
    # Размер списка процессов (не считая информацию о самом экспортере)
    process_limit: 5
    # Критерий сортировки процессов
    sort_by: "cpu_usage" # / "memory" / "virtual_memory" / "start_time" / "run_time"

  disks:
    enable: true

  components:
    enable: true

  network:
    enable: true

  memory:
    enable: true
```

---

## Планы развития
Не всё что находистя в списке будет реализовано на 100%. Некоторые 
фичи так могут и не быть реализованы

- [ ] Форматы
  - [X] Поддержка Line Protocol для InfluxDB v2
  - [X] Поддержка JSON 
  - [ ] Поддержка стандарта OpenMetrics 
- [ ] Функционал
    - [ ] Конфигурация
      - [ ] Добавление pull модели
      - [ ] Добавление списка игнорируемых сетевых интерфейсов
      - [ ] Добавление игнорируемый mount-поинтов для диска
      - [X] Возможность подставлять в конфигурацию переменные окружения вашей системы
      - [X] Перейти с .toml в .yaml
    - [ ] Метрики
      - [ ] Добавить топ-N процессов в системе по использованию процессора, памяти и т.д. в одном теле метрик
    - [ ] Поддерживаемые платформы
      - [ ] Windows Server
      - [ ] Linux
      - [ ] Микроконтроллеры (**no_std**, *версии устройств будут конкретизированы позже*)
        - [ ] Esp32 