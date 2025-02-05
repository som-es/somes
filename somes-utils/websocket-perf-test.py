import asyncio
import websockets
import random
import requests

WS_URL = "ws://somes.at:3000/quiz_room"  
NUM_CONNECTIONS = 800

async def connect_and_send(index):
    await asyncio.sleep((random.random() + 0.02) * 50) 
    print(f"wake up {index}")
    x = requests.get("http://somes.at:3000/delegates")
    x.json()
    print(f"JSON ready: {index}")
    async with websockets.connect(WS_URL, open_timeout=60) as websocket:
        await websocket.send("b")
        response = await websocket.recv()
        print(f"Connection {index}: {response}")

        # await websocket.send("n")
        # response = await websocket.recv()
        # print(f"Connection {index}: {response}")
        await asyncio.sleep(60)  

async def main():
    tasks = [connect_and_send(i) for i in range(NUM_CONNECTIONS)]
    await asyncio.gather(*tasks)

if __name__ == "__main__":
    asyncio.run(main())