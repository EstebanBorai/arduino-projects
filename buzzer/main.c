int buzzer_out_pin = 3;

void setup() {
  pinMode(buzzer_out_pin, OUTPUT);
}

void loop() {
  delay(500);
  tone (buzzer_out_pin, 1000, 1000);
  delay(500);
  noTone(3);
}
