Задание: Эмулируем протокол TokenRing. Во время инициализации получаем от пользователя число и запускаем N потоков (goroutine). Каждый узел i должен быть связан очередями (channel) с двумя другими и получать данные от i-1 и посылать в i+1 узел. 

Основной поток должен иметь возможность отправить сообщение только одному узлу в виде экземпляра структуры/класса Token (data:string, recipient:int, ttl:int). Узлы передают токен по цепочке, пока сообщение не достигнет адресата сообщения или не истечет время жизни (ttl-=1 на каждой пересылке).

Базовый язык - Go (golang). 

Как запустить:
1) Скачать файл .go
2) Через командную строку в папке с файлом создать go.mod
Например, написать: "go mod init example.com/mymodule"
3) Запустить программу: "go run ."

Можно ввести количество узлов, номер получателя, сообщение и TTL

Чтобы завершить работу, нажмите "Ctrl + C"