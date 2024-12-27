import { invoke } from "@tauri-apps/api/core";

export interface todo {
  id: number;
  title: string;
  description: string;
}

export const todoApi = {
  async getAllTodos(): Promise<todo[]> {
    return invoke("fetch_todos");
  },

  async createTodo(todo: Omit<todo, "id">): Promise<void> {
    return invoke("create_todo", { todo });
  },

  async updateTodo(id: number, todo: Omit<todo, "id">): Promise<void> {
    return invoke("update_todo", { id, todo });
  },

  async deleteTodo(id: number): Promise<void> {
    return invoke("delete_todo", { id });
  },
};
