import sys

def show_menu():
    print("\nToDo List App")
    print("1. Add task")
    print("2. List tasks")
    print("3. Remove task")
    print("4. Exit")

def add_task(tasks):
    task = input("Enter task: ")
    tasks.append(task)

def list_tasks(tasks):
    for i, task in enumerate(tasks, start=1):
        print(f"{i}. {task}")

def remove_task(tasks):
    list_tasks(tasks)
    index = int(input("Enter task number to remove: ")) - 1
    tasks.pop(index)

def main():
    tasks = []

    while True:
        show_menu()
        choice = int(input("Enter your choice: "))

        if choice == 1:
            add_task(tasks)
        elif choice == 2:
            list_tasks(tasks)
        elif choice == 3:
            remove_task(tasks)
        elif choice == 4:
            sys.exit(0)
        else:
            print("Invalid choice. Please try again.")

if __name__ == "__main__":
    main()
