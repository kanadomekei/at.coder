package main

import (
	"fmt"
	"sync"
	"time"
)

func main() {
	fmt.Println("=== Goのゴルーチンとチャネル ===")

	// 基本的なゴルーチン
	go sayHello()
	go sayGoodbye()
	time.Sleep(1 * time.Second) // メインゴルーチンが終了しないように待機

	// チャネルの基本
	ch := make(chan string)
	go func() {
		ch <- "Hello from goroutine!"
	}()
	
	msg := <-ch
	fmt.Println(msg)

	// バッファ付きチャネル
	bufferedCh := make(chan int, 3)
	bufferedCh <- 1
	bufferedCh <- 2
	bufferedCh <- 3
	
	fmt.Printf("受信: %d\n", <-bufferedCh)
	fmt.Printf("受信: %d\n", <-bufferedCh)

	// ワーカープール
	jobs := make(chan int, 5)
	results := make(chan int, 5)

	// ワーカーを3つ起動
	for w := 1; w <= 3; w++ {
		go worker(w, jobs, results)
	}

	// ジョブを送信
	for j := 1; j <= 5; j++ {
		jobs <- j
	}
	close(jobs)

	// 結果を受信
	for a := 1; a <= 5; a++ {
		<-results
	}

	// select文
	ch1 := make(chan string)
	ch2 := make(chan string)

	go func() {
		time.Sleep(1 * time.Second)
		ch1 <- "チャネル1"
	}()

	go func() {
		time.Sleep(2 * time.Second)
		ch2 <- "チャネル2"
	}()

	for i := 0; i < 2; i++ {
		select {
		case msg1 := <-ch1:
			fmt.Printf("受信: %s\n", msg1)
		case msg2 := <-ch2:
			fmt.Printf("受信: %s\n", msg2)
		}
	}

	// sync.WaitGroupの使用
	var wg sync.WaitGroup
	
	for i := 1; i <= 3; i++ {
		wg.Add(1)
		go func(id int) {
			defer wg.Done()
			fmt.Printf("ワーカー %d が動作中\n", id)
			time.Sleep(time.Second)
		}(i)
	}
	
	wg.Wait()
	fmt.Println("すべてのワーカーが完了しました")
}

func sayHello() {
	fmt.Println("Hello!")
}

func sayGoodbye() {
	fmt.Println("Goodbye!")
}

func worker(id int, jobs <-chan int, results chan<- int) {
	for j := range jobs {
		fmt.Printf("ワーカー %d がジョブ %d を開始\n", id, j)
		time.Sleep(time.Second)
		fmt.Printf("ワーカー %d がジョブ %d を完了\n", id, j)
		results <- j * 2
	}
}