package main

import (
	"fmt"
	"sort"
	"strings"
	"sync"
	"time"
)

func main() {
	fmt.Println("=== Go実践的なサンプル ===")

	// 単語カウンター
	text := "hello world wonderful world hello"
	wordCount := countWords(text)
	fmt.Printf("単語カウント: %v\n", wordCount)

	// 温度変換
	celsius := 25.0
	fahrenheit := celsiusToFahrenheit(celsius)
	fmt.Printf("%.1f°C = %.1f°F\n", celsius, fahrenheit)

	// 並行処理による計算
	numbers := []int{1, 2, 3, 4, 5, 6, 7, 8, 9, 10}
	sum := concurrentSum(numbers)
	fmt.Printf("並行計算による合計: %d\n", sum)

	// Webサーバーのシミュレーション
	simulateWebServer()

	// データ処理パイプライン
	processDataPipeline()
}

func countWords(text string) map[string]int {
	words := strings.Fields(strings.ToLower(text))
	count := make(map[string]int)
	
	for _, word := range words {
		count[word]++
	}
	
	return count
}

func celsiusToFahrenheit(celsius float64) float64 {
	return celsius*9/5 + 32
}

func concurrentSum(numbers []int) int {
	const numWorkers = 3
	jobs := make(chan int, len(numbers))
	results := make(chan int, len(numbers))
	
	// ワーカーを起動
	for i := 0; i < numWorkers; i++ {
		go func() {
			sum := 0
			for num := range jobs {
				sum += num
			}
			results <- sum
		}()
	}
	
	// ジョブを送信
	for _, num := range numbers {
		jobs <- num
	}
	close(jobs)
	
	// 結果を集計
	total := 0
	for i := 0; i < numWorkers; i++ {
		total += <-results
	}
	
	return total
}

func simulateWebServer() {
	fmt.Println("\nWebサーバーシミュレーション:")
	
	requests := []string{"GET /", "GET /api/users", "POST /api/login", "GET /static/style.css"}
	
	var wg sync.WaitGroup
	
	for _, req := range requests {
		wg.Add(1)
		go func(request string) {
			defer wg.Done()
			handleRequest(request)
		}(req)
	}
	
	wg.Wait()
	fmt.Println("すべてのリクエストを処理しました")
}

func handleRequest(request string) {
	// リクエスト処理のシミュレーション
	processingTime := time.Duration(100+len(request)*10) * time.Millisecond
	time.Sleep(processingTime)
	
	fmt.Printf("処理完了: %s (処理時間: %v)\n", request, processingTime)
}

func processDataPipeline() {
	fmt.Println("\nデータ処理パイプライン:")
	
	// データ生成
	data := []int{5, 2, 8, 1, 9, 3, 7, 4, 6}
	fmt.Printf("元データ: %v\n", data)
	
	// パイプライン処理
	filtered := filter(data, func(x int) bool { return x > 3 })
	fmt.Printf("フィルター後: %v\n", filtered)
	
	mapped := mapInts(filtered, func(x int) int { return x * x })
	fmt.Printf("マップ後: %v\n", mapped)
	
	sort.Ints(mapped)
	fmt.Printf("ソート後: %v\n", mapped)
	
	sum := reduce(mapped, 0, func(acc, x int) int { return acc + x })
	fmt.Printf("合計: %d\n", sum)
}

func filter(data []int, predicate func(int) bool) []int {
	var result []int
	for _, item := range data {
		if predicate(item) {
			result = append(result, item)
		}
	}
	return result
}

func mapInts(data []int, mapper func(int) int) []int {
	result := make([]int, len(data))
	for i, item := range data {
		result[i] = mapper(item)
	}
	return result
}

func reduce(data []int, initial int, reducer func(int, int) int) int {
	result := initial
	for _, item := range data {
		result = reducer(result, item)
	}
	return result
}

// 従業員管理システム
type Employee struct {
	ID       int
	Name     string
	Position string
	Salary   float64
}

type Company struct {
	employees map[int]Employee
	mutex     sync.RWMutex
}

func NewCompany() *Company {
	return &Company{
		employees: make(map[int]Employee),
	}
}

func (c *Company) AddEmployee(emp Employee) {
	c.mutex.Lock()
	defer c.mutex.Unlock()
	c.employees[emp.ID] = emp
}

func (c *Company) GetEmployee(id int) (Employee, bool) {
	c.mutex.RLock()
	defer c.mutex.RUnlock()
	emp, exists := c.employees[id]
	return emp, exists
}

func (c *Company) ListEmployees() []Employee {
	c.mutex.RLock()
	defer c.mutex.RUnlock()
	
	var employees []Employee
	for _, emp := range c.employees {
		employees = append(employees, emp)
	}
	return employees
}