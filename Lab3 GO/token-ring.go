package main

import (
	"fmt"
	"time"
)

type Token struct {
	Data      string
	Recipient int
	TTL       int // Time to live
}

func node(id int, in <-chan Token, out chan<- Token) {
	for {
		token := <-in
		fmt.Printf("Узел %d: получен токен: %+v\n", id, token)

		if token.Recipient == id {
			fmt.Printf("Узел %d получил сообщение: %s\n", id, token.Data)
			continue
		}
		if token.TTL > 0 {
			token.TTL--
			out <- token
		} else {
			fmt.Printf("Узел %d: TTL истекло для сообщения: %s\n", id, token.Data)
		}
	}
}

func main() {
	var numNodes int
	fmt.Print("Введите количество узлов: ")
	fmt.Scanln(&numNodes)
	if numNodes < 0 {
		fmt.Println("Неверный ввод, количество узлов должно быть целым положительным числом")
		return
	}


	channels := make([]chan Token, numNodes)
	for i := range channels {
		channels[i] = make(chan Token)
	}

	for i := 0; i < numNodes-1; i++ {
		go node(i, channels[i], channels[i+1])
	}
	go node(numNodes-1, channels[numNodes-1], channels[1])

	var recipient int
	fmt.Print("Введите номер узла-получателя (от 0 до ", numNodes-1, "): ")
	fmt.Scanln(&recipient)
	if recipient < 0 || recipient > numNodes-1 {
		fmt.Println("Неверный ввод")
		return
	}

	var message string
	fmt.Print("Введите сообщение для отправки: ")
	fmt.Scanln(&message)

	var ttl int
	fmt.Print("Введите TTL (Time To Live): ")
	fmt.Scanln(&ttl)

	initialToken := Token{Data: message, Recipient: recipient, TTL: ttl}
	channels[0] <- initialToken

	//Программа закроется через 5 секунд
	time.Sleep(5 * time.Second)
}