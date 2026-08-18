# Persona Exporter
## Описание
Легковесный экспортер метрик
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
### Первичная настройка
Основной файл конфигурации **config.toml** по умолчанию хранится в ```/etc/persona-exporter/config.toml```.
Рабочую директорию можно изменить через переменную окружения ```PERSONA_EXPORTER_CONFIG_DIR```.
К примеру 
```bash
export PERSONA_EXPORTER_CONFIG_DIR="~/.config/persona-exporter/" 
```
### Собираемые метрики
Объемы (к примеру, объяем оперативной памяти, дискового пространства) предоставляются в байтах.
Перед
#### ```Система```
- **Имя** (`name`): Название вашей ОС / Дистрибутива
- **Версия ядра** (`kernel_version`): Версия вашего ядра
- **Полная версия системы** (`kernel_long_version`): Название платформы + Версия ядра 
- **Имя дистрибутива** (`distribution_id`): Название конкретного дистрибутива
- **Родители дистрибутива** (`distribution_id_like`): Родители дистрибутива. К примеру если имя 
- дистрибутива "ubuntu" то родитель "debian" так как Ubuntu берет за основу Debian.
#### ```Процессы```
#### ```Оперативная Память```
#### ```Диск```
#### ```Сеть```
#### ```Датчики```
#### ```Процессор```
#### ```Время```
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
### Конфигурация
Теперь подробнее про конфигурацию, как упоминалось ранее по умолчанию файл конфигурации
находится в `/etc/persona-exporter/config.toml` и можно изменить директорию с помощью
переменной окружения `PERSONA_EXPORTER_CONFIG_DIR`.
Ниже будут описаны все поля конфигурации. Для наглядности мы опишем 
вполне работающую конфигурацию для Influx DB v2
```toml
[agent]
# Интервал отправки метрик
send_interval = 10
# Формат данных которые будут отправляться на сервер.
# "json" / "line_protocol"
data_type = "line_protocol"

[server]
# Целевой сервер (url)
url = "http://localhost:8086/api/v2/write"
# Заголовки которые будут переданы в тело запрос, нужно к примеру
# для токенов авторизации
http_headers = [
    { key = "Authorization", value = "Token ${INFLUX_DB_TOKEN}" },
    { key = "Content-Type", value = "text-plain; charset=utf-8" }
]
# Дополнительно устанавливаем get-параметры которые требует Influx DB v2
get_params = [
    { key = "org", value = "my-great-company" },
    { key = "bucket", value = "org-server-metrics" },
    { key = "precision", value = "ns" }
]

# Далее по необходиомости описывает конкретные категории метрик
[metrics.cpu]
enable = true

[metrics.system]
enable = true

[metrics.processes]
enable = true
# Размер списка процессов
process_limit = 5

[metrics.disks]
enable = true

[metrics.components]
enable = true

[metrics.network]
enable = true

[metrics.memory]
enable = true
```
## Планы развития

- [ ] Форматы
  - [X] Поддержка Line Protocol для InfluxDB
  - [X] Поддержка JSON 
  - [ ] Поддержка OpenMetrics 
- [ ] Функционал
    - [ ] Конфигурация
      - [ ] Добавление pull модели
      - [ ] Добавление специфичных настроек метрик. 
        К примеру добавление игнорируемых интерфейсов при сборе метрик по сети и т.д.
      - [X] Возможность подставлять в конфигурацию переменные окружения вашей системы
    - [ ] Метрики
      - [ ] Добавить топ-N процессов в системе по использованию процессора, памяти и т.д.
      - [ ] Оптимизировать работу с памятью при сборе 
      - [ ] Добавить возможность преобразования метрик. К примеру вместо сырых байт некоторые метрики будут преобразоваться 
      в понятный формат 
    - [ ] Поддерживаемые платформы
      - [ ] Windows (**В далеком будущем**)
        - [ ] Windows Server
        - [ ] Windows 10
        - [ ] Windows 11
      - [ ] Linux
        - [ ] Репозитории пакетов
          - [ ] `APT` (Debian, Ubuntu)
          - [ ] `AUR` (ArchLinux)
          - [ ] `nixpkgs` (NixOS)
          - [ ] `EPEL` (RadHat, AlmaLinux)
          - [ ] `Copl` (RadHat, Fedora)
        - [ ] Дистрибутивы
          - [X] Ubuntu
          - [X] Debian
          - [ ] NixOS
          - [ ] RedHat
          - [ ] Fedora
          - [ ] AlmaLinux
      - [ ] MacOS (**В далеком будущем**)
      - [ ] Микроконтроллеры (**no_std**, *версии устройств будут конкретизированы позже*)
        - [ ] Raspberry Pi
        - [ ] Esp32 
