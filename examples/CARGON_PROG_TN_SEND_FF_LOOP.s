// This program sends 0xFF to RECIPIENT_ADDR and waits for an answer. If the answer is 0xFF it sends it again and so on until iteration 255.
// If the answer is not 0xFF then it halts. Keep in mind that 0 answer is the same as no packet due to adapter limitations, so the program will retry on 0 answer.
// If it halts, r1 will contain the last number received over the network.
// Set RECIPIENT_ADDR to this CPU's address to send packets to itself.

@define RECIPIENT_ADDR 42
@define NUMBER_TO_SEND 0xFF
@define MAX_ITERATIONS 255
@define START_ITERATION 0

// Send anything to TN_NEXT_DATA_PORT to get next data byte 
@define TN_NEXT_DATA_PORT $0
// Send TN command to this port
@define TN_COMMAND_PORT $1
// Send data to this port to include data in the packet
@define TN_WRITE_DATA_PORT $2
// Send recepient address to this port to send the packet
@define TN_RECEPIENT_ADDR_PORT $3

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

// Output the received number to this port for transparency
@define OUTPUT_PORT $4

lim r0 @TN_COMMAND_ON
pst @TN_COMMAND_PORT // Turn the TN interface on

lim r0 @NUMBER_TO_SEND
pst @TN_WRITE_DATA_PORT
lim r0 @RECIPIENT_ADDR
pst @TN_RECEPIENT_ADDR_PORT

.receive_loop
pld @TN_STATUS_PORT
lim r1 @TN_MORE_PACKETS_BIT
and r1
brc eq .receive_loop // If bit 2 is not set, wait for a packet to be received

lim r0 @TN_COMMAND_NEXT_PACKET
pst @TN_COMMAND_PORT // Next packet command
pst @TN_NEXT_DATA_PORT // Write anything to TN_NEXT_DATA_PORT to trigger next data byte command
pld @TN_READ_DATA_PORT // Read data from input port into accumulator
pst @OUTPUT_PORT // Output the received number

.halt 
hlt