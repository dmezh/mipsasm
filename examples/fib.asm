entry:  addi $1, $0, 47   #  int x = 47; // get 47th fibonacci number

fib:    addi $2, $0, 0    #  int number1 = 0;
        addi $3, $0, 1    #  int number2 = 1;
        addi $4, $0, 1    #  int next = 1;
        addi $1, $1, -1   #  x -= 1;

        add  $5, $0, $0   #  for (int i = 0; i < x; i++)
loop:   beq  $1, $5, end  #
        addi $5, $5, 1    #  {
        add  $4, $2, $3   #     next = number1 + number2;
        add  $2, $0, $3   #     number1 = number2;
        add  $3, $0, $4   #     number2 = next;
        j    loop         #  }

end:    sw   $4, 84($0)   #  *84 = next;
