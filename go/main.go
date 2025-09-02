package main
import (
 "context"
 "database/sql"
 "encoding/json"
 "log"
 "net/http"
 "os"
 "time"
 "github.com/go-redis/redis/v8"
 _ "github.com/lib/pq"
)
var (
 db     *sql.DB
 rdb    *redis.Client
 ctx    = context.Background()
 client = http.Client{Timeout: 2 * time.Second}
)
type Order struct {
 UserID    string  `json:"user_id"`
 ProductID string  `json:"product_id"`
 Amount    float64 `json:"amount"`
 Currency  string  `json:"currency"`
}
func handleOrder(w http.ResponseWriter, r *http.Request) {
 defer recoverPanic()
 var order Order
 if err := json.NewDecoder(r.Body).Decode(&order); err != nil {
  http.Error(w, "invalid payload", 400)
  return
 }
 // Redis GET
 cacheKey := "user:" + order.UserID
 _, _ = rdb.Get(ctx, cacheKey).Result() // we don't need the result here
 // External API call
 rate, err := getCurrencyRate(order.Currency)
 if err != nil {
  log.Println("currency API failed:", err)
 }
 // DB INSERT
 _, err = db.Exec(`INSERT INTO orders (user_id, product_id, amount, currency) VALUES ($1, $2, $3, $4)`,
  order.UserID, order.ProductID, order.Amount*rate, order.Currency)
 if err != nil {
  log.Println("DB error:", err)
  http.Error(w, "db error", 500)
  return
 }
 w.WriteHeader(200)
}
func getCurrencyRate(currency string) (float64, error) {
 req, _ := http.NewRequest("GET", "http://external-currency-api/rate?to="+currency, nil)
 resp, err := client.Do(req)
 if err != nil || resp.StatusCode != 200 {
  return 1.0, err
 }
 defer resp.Body.Close()
 var data struct{ Rate float64 }
 json.NewDecoder(resp.Body).Decode(&data)
 return data.Rate, nil
}
func recoverPanic() {
 if r := recover(); r != nil {
  log.Println("Recovered:", r)
 }
}
func main() {
 var err error
 db, err = sql.Open("postgres", os.Getenv("POSTGRES_DSN"))
 if err != nil {
  log.Fatal(err)
 }
 rdb = redis.NewClient(&redis.Options{
  Addr: os.Getenv("REDIS_ADDR"),
 })
 http.HandleFunc("/order", handleOrder)
 log.Fatal(http.ListenAndServe(":8080", nil))
}