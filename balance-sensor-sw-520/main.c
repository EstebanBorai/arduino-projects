int balance = 0;

void setup() {
  // initialize Serial port
  Serial.begin(9600);
  
  // The SW-520 sensor must be plugged to the
  // analog input on PINs 0 and 1
  // Sets PIN A0 on input mode
  pinMode(A0, INPUT);
  // Sets PIN A1 on output mode
  pinMode(A1, OUTPUT);
  // Sets PIN A1 on HIGH
  digitalWrite(A1, HIGH);
}

void loop() {
  // Sets balance to the current value of Analog
  // PIN A0
  balance = analogRead(A0);
  // Prints the value to the Serial console
  // available on the top right corner of the
  // Arduino IDE
  Serial.println(balance);
  // Sleeps for 100ms
  delay(100);
}
