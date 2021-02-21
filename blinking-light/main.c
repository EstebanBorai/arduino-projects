void setup() {
  // pinMode defines the behavior of an input pin
  pinMode(13, OUTPUT);
}

void loop() {
  // Turns on the light on PIN 13
  digitalWrite(13, HIGH);
  // Sleeps for 2secs
  delay(2000);
  // Turns off the LED on PIN 13
  digitalWrite(13, LOW);
  // Sleeps for 2secs
  delay(2000);
}

