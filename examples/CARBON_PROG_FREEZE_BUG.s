// This program tests that TickNet Node functions correctly in edge cases

// Address of the node that is connected to the CPU
@define RECIPIENT_ADDR 42
// Amount of empty packets to send to fill all of the node's memory
@define AMOUNT_OF_EMPTY_PACKETS_TO_SEND 32
@define ALL_TESTS_PASSED_CODE 255
// Holds the number of the index of the test that failed (from 1 to N) or 255 if all tests passed
@define EXIT_CODE_PORT $4
// Holds the error code of a concrete test that failed
@define ERROR_CODE_PORT $5
// Holds the received value from TN Node that was discovered to be wrong during comparison
@define MISMATCHED_VALUE_PORT $6
// Holds the number of the currently running test (from 1 to N), 0 is setup
@define CURRENT_TEST_PORT $7

// ** TickNet <-> Carbon Protocol ** //
// Send TN command to this port
@define TN_COMMAND_PORT $1
// Send data to this port to include data in the packet
@define TN_WRITE_DATA_PORT $2
// Send recipient address to this port to send the packet
@define TN_RECIPIENT_ADDR_PORT $3

// Read node status from this port. Bit 1 is set if the node is online, bit 2 is set if there are packets to receive
@define TN_STATUS_PORT $1
// Read data from this port to get the next data byte
@define TN_READ_DATA_PORT $2
// Read the remaining number of data bytes in the currently read packet
@define TN_LENGTH_PORT $0

// Send these commands to TN_COMMAND_PORT
// Turn the interface on
@define TN_COMMAND_ON 1
// Turn the interface off
@define TN_COMMAND_OFF 2
// Get the next packet
@define TN_COMMAND_NEXT_PACKET 4

// AND this value and TN_STATUS_PORT to check if there are more packets to receive
@define TN_MORE_PACKETS_BIT 1
// AND this value and TN_STATUS_PORT to check if the node is online
@define TN_ONLINE_BIT 2

// Maximum payload length per packet
@define TN_MAX_BYTES_PER_PACKET 2

// Clear the ports for transparency
lim r0 0
pst @EXIT_CODE_PORT
pst @ERROR_CODE_PORT
pst @MISMATCHED_VALUE_PORT
pst @CURRENT_TEST_PORT 

lim r0 @TN_COMMAND_ON
pst @TN_COMMAND_PORT // Turn the TN interface on

brc jmp .test_3_start // Uncomment to jump to the required test immediately

// ** Test 1: Send a packet with 1 byte to self ** //
lim r0 1
pst @CURRENT_TEST_PORT

@define TEST_1_NUMBER_TO_SEND 255
.test_1_start
lim r0 @TEST_1_NUMBER_TO_SEND
pst @TN_WRITE_DATA_PORT
lim r0 @RECIPIENT_ADDR
pst @TN_RECIPIENT_ADDR_PORT // Send the number to the recipient
nop 
nop 
nop // Wait 8 + 24 = 32 (>25) redstone ticks for the packet to be sent

.test_1_receive_loop
pld @TN_STATUS_PORT
lim r1 @TN_MORE_PACKETS_BIT
and r1
brc eq .test_1_receive_loop // If more packets bit is not set, wait for a packet to be received

lim r0 @TN_COMMAND_NEXT_PACKET
pst @TN_COMMAND_PORT // Next packet command
nop
nop
nop
nop
nop
nop // Wait for 8 + 48 = 56 (>55) ticks for the received data to be ready

// Test 1 assertion 1 length
pld @TN_LENGTH_PORT
lim r1 1
cmp r1
brc eq .test_1_assert_2 // Assert remaining payload length is 1

// Test 1 error 1
lim r0 1
pst @ERROR_CODE_PORT
pst @EXIT_CODE_PORT
brc jmp .off

// Test 1 assertion 2 data
.test_1_assert_2
pld @TN_READ_DATA_PORT // Read data from input port into accumulator
lim r1 @TEST_1_NUMBER_TO_SEND
cmp r1
brc eq .test_1_assert_3 // Assert received data = sent data

// Test 1 error 2
lim r0 2
pst @ERROR_CODE_PORT
lim r0 1
pst @EXIT_CODE_PORT
brc jmp .off

// Test 1 assertion 3 length
.test_1_assert_3
pld @TN_LENGTH_PORT
lim r1 0
cmp r1
brc eq .test_2_start // Assert remaining payload length is 0

// Test 1 error 3
lim r0 3
pst @ERROR_CODE_PORT
lim r0 1
pst @EXIT_CODE_PORT
brc jmp .off

// ** Test 2: Send and receive an empty packet ** //

.test_2_start
lim r0 2
pst @CURRENT_TEST_PORT

// ** Test 3: Send and receive a packet with maximum number of bytes ** //

.test_3_start
lim r0 3
pst @CURRENT_TEST_PORT

lim r1 @TN_MAX_BYTES_PER_PACKET

.test_3_loop // Fill the packet with maximum number of bytes
    rld r1
    pst @TN_WRITE_DATA_PORT
    dec r1
    lim r0 0
    cmp r1
    brc neq .test_3_loop

lim r0 @RECIPIENT_ADDR
pst @TN_RECIPIENT_ADDR_PORT // Send the number to the recipient
nop 
nop 
nop // Wait 8 + 24 = 32 (>25) redstone ticks for the packet to be sent

.test_3_receive_loop
    pld @TN_STATUS_PORT
    lim r1 @TN_MORE_PACKETS_BIT
    and r1
    brc eq .test_3_receive_loop // If more packets bit is not set, wait for a packet to be received

lim r0 @TN_COMMAND_NEXT_PACKET
pst @TN_COMMAND_PORT // Next packet command
nop
nop
nop
nop
nop
nop // Wait for 8 + 48 = 56 (>55) ticks for the received data to be ready

// Test 3 assertions 24-1
lim r1 @TN_MAX_BYTES_PER_PACKET
.test_3_assert_loop
    // Test 3 assert length
    pld @TN_LENGTH_PORT
    cmp r1 // r0 is the received length (could be wrong), r1 is the iteration number (from 24 to 1)
    brc eq .test_3_assert_data // assert r1 == length

    // Test 3 error length code [r0] (for example 0b0001_1000 for iteration 1 if length is wrong as first length is 24)
    .test_3_error_length
    pst @MISMATCHED_VALUE_PORT
    rld r1
    pst @ERROR_CODE_PORT
    lim r0 3
    pst @EXIT_CODE_PORT    
    brc jmp .off

    .test_3_assert_data
    pld @TN_READ_DATA_PORT // Read data from input port into accumulator
    cmp r1
    brc eq .test_3_assert_loop_end // assert r1 == data

    // Test 3 error code 0b1000_0000 + [r0] (for example 0b1001_1000 for iteration 1 if data is wrong as first data byte is 24)
    .test_3_error_data
    pst @MISMATCHED_VALUE_PORT
    rld r1
    lim r3 0x80
    bor r3 // set MSB of error code in acc to 1 to indicate data mismatch
    pst @ERROR_CODE_PORT
    lim r0 3
    pst @EXIT_CODE_PORT
    brc jmp .off

    .test_3_assert_loop_end
    dec r1
    lim r0 0
    cmp r1
    brc neq .test_3_assert_loop

// ** Test 4: Send and receive the maximum amount of empty packets the node can hold ** //
// Node V1.1.5 can store 64 bytes of received data
// 1 byte for sender address + 1 byte for length = 2 bytes per packet
// 64 bytes / 2 bytes per packet = 32 packets

lim r0 4
pst @CURRENT_TEST_PORT

// ** Test 5: Turn the node off and on repeatedly ** //

lim r0 5
pst @CURRENT_TEST_PORT

// ** End of test suite ** //

lim r0 @ALL_TESTS_PASSED_CODE
pst @EXIT_CODE_PORT

.off 
lim r0 @TN_COMMAND_OFF
pst @TN_COMMAND_PORT // Turn the TN interface off
hlt
