
<h1 align="center">
      deadlock-esp
</h1>

<p align="center">
Добавьте этот репозиторий в избранное, чтобы не пропустить новые обновления.<br>
</p>

> [!WARNING]
> Использование чит-программ строго запрещено Габеном. Это может привести к блокировке вашего аккаунта!<br>
> Если вам не плевать на ваш аккаунт, то не используйте любые читы, включая этот

> [!WARNING]
> The use of cheat programs is strictly prohibited by Gaben. This may result in your account being banned!<br>
> f you care about your account, then do not use any cheats, including this one

## Настройки

+ Аимбот
   - Цели
      - Игроки
      - Души, крипы
   - Учитывает скорость цели
   - Контроль отдачи (RCS)
   - Настраиваемый FOV
      - Дистанция
      - Цвет
   - Настрока для максимальной дистанции
+ Радар
   - Маштаб
   - Цвета
   - Размер, позиция
+ ESP игроков
   - Боксы (прямоугольник игрока)
      - Тип обводки
         - Обычный
         - Закругленный
         - Углы
      - Обводка
      - Тень обводки
      - Настройка цветов и тени обводки
   - Голова
   - Шкала здоровья игрока (healthbar)
   - Надписи
      - Отображание:
         - Здоровье
         - Название героя
         - Дистанция в метрах
      - Расположение
      - Контрастность
      - Размер шрифта
+ Файл конфигурации
   - Сохранение / Загрузка
+ Список наблюдателей
   - Показывает кто за вами смотрит
   - Показывает кто еще смотрит с вами
+ Локализация
   - Русский
   - Китайский
   - Английский

## Прочие мелочи
   - Цикл оверлея не работает, когда окно игры не активно
   - Не используется WriteProcessMemory
   - События мыши отсылаются из другого процесса
   - Авто обновление указателей из памяти модуля игры
   - В коде есть GlobalVars и Abilities, но не используются. Можете сами что-то запилить

## Todo

- [ ] Выбор и сохранение конфигов по названию
- [ ] Авто парирование
- [ ] Аим на разные кости
- [ ] Чекбокс для списка наблюдателей
- [ ] Авто загрузка последнего конфига


<hr>

# Запуск 

1. Меняем имя приложения в [toml файле](https://github.com/Loara228/deadlock-esp/blob/master/Cargo.toml)

```txt 
name = "{название программы}"
```

2. Компилируем проект:

```txt
cargo build --release
```

3. Запускаем игру
4. Запускаем чит

```txt
{название программы} --offsets
```

5. Для открытия/закрытия используется клавиша <kbd>HOME</kbd>

## Offsets

<div align="left">
<b>
      <a href="https://github.com/Loara228/deadlock-esp/blob/master/offsets/client_dll.cs">cs💜</a> | 
      <a href="https://github.com/Loara228/deadlock-esp/blob/master/offsets/client_dll.hpp">cpp💀</a> | 
      <a href="https://github.com/Loara228/deadlock-esp/blob/master/offsets/client_dll.rs">rs🦀</a>
      (больше не обновляю)
</b>
</div>

## Превью

![](images/1.png)![](preview)

![](images/spec.png)![](preview)

![](images/3.png)![](preview)

![](images/4.png)![](preview)

![](images/5.png)![](preview)

![](images/esp1.png)![](preview)

<div align = "center">
<img src="https://github.com/user-attachments/assets/5aa2dd1b-b106-4831-9c70-df3a672da18b" height=" 800"/>
</div>

> <a href="https://www.youtube.com/watch?v=3nJs6GPmEZs"><sub><sub>💪старый бог💪</sup></sub></a>

