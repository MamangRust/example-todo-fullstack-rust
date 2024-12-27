import { useState, useEffect } from 'react';
import { todo, todoApi } from './api/todo';

function App() {
  const [todos, setTodos] = useState<todo[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [newTodo, setNewTodo] = useState({ title: '', description: '' });
  const [editingTodo, setEditingTodo] = useState<todo | null>(null);

  useEffect(() => {
    fetchTodos();
  }, []);

  const fetchTodos = async () => {
    try {
      setLoading(true);
      const data: todo[] = await todoApi.getAllTodos();
      setTodos(data);
      setError(null);
    } catch (err: any) {
      setError(`Failed to fetch todos: ${err.message || err}`);
      console.error('Error:', err);
    } finally {
      setLoading(false);
    }
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      setLoading(true);
      await todoApi.createTodo(newTodo);
      setNewTodo({ title: '', description: '' });
      await fetchTodos();
    } catch (err: any) {
      setError(`Failed to create todo: ${err.message || err}`);
      console.error('Error:', err);
    } finally {
      setLoading(false);
    }
  };

  const handleUpdate = async (id: number) => {
    if (!editingTodo) return;
    try {
      setLoading(true);
      await todoApi.updateTodo(id, editingTodo);
      setEditingTodo(null);
      await fetchTodos();
    } catch (err: any) {
      setError(`Failed to update todo: ${err.message || err}`);
      console.error('Error:', err);
    } finally {
      setLoading(false);
    }
  };

  const handleDelete = async (id: number) => {
    try {
      setLoading(true);
      await todoApi.deleteTodo(id);
      await fetchTodos();
    } catch (err: any) {
      setError(`Failed to delete todo: ${err.message || err}`);
      console.error('Error:', err);
    } finally {
      setLoading(false);
    }
  };

  if (loading) {
    return <div className="text-center p-4">Loading...</div>;
  }

  return (
    <div className="container mx-auto p-4">
      <h1 className="text-2xl font-bold mb-4">Todo App</h1>

      {error && (
        <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
          {error}
        </div>
      )}

      <form onSubmit={handleSubmit} className="mb-6">
        <div className="mb-4">
          <input
            type="text"
            placeholder="Title"
            value={newTodo.title}
            onChange={(e) => setNewTodo({ ...newTodo, title: e.target.value })}
            className="border p-2 mr-2 rounded"
            required
          />
          <input
            type="text"
            placeholder="Description"
            value={newTodo.description}
            onChange={(e) => setNewTodo({ ...newTodo, description: e.target.value })}
            className="border p-2 mr-2 rounded"
            required
          />
          <button
            type="submit"
            className="bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600"
            disabled={loading}
          >
            Add Todo
          </button>
        </div>
      </form>

      <div>
        {todos.map((todo) => (
          <div key={todo.id} className="border p-4 mb-2 rounded shadow">
            {editingTodo?.id === todo.id ? (
              <div>
                <input
                  type="text"
                  value={editingTodo.title}
                  onChange={(e) => setEditingTodo({ ...editingTodo, title: e.target.value })}
                  className="border p-2 mr-2 rounded"
                />
                <input
                  type="text"
                  value={editingTodo.description}
                  onChange={(e) => setEditingTodo({ ...editingTodo, description: e.target.value })}
                  className="border p-2 mr-2 rounded"
                />
                <button
                  onClick={() => handleUpdate(todo.id)}
                  className="bg-green-500 text-white px-4 py-2 rounded mr-2 hover:bg-green-600"
                  disabled={loading}
                >
                  Save
                </button>
                <button
                  onClick={() => setEditingTodo(null)}
                  className="bg-gray-500 text-white px-4 py-2 rounded hover:bg-gray-600"
                  disabled={loading}
                >
                  Cancel
                </button>
              </div>
            ) : (
              <div>
                <h3 className="font-bold">{todo.title}</h3>
                <p className="text-gray-600">{todo.description}</p>
                <div className="mt-2">
                  <button
                    onClick={() => setEditingTodo(todo)}
                    className="bg-yellow-500 text-white px-4 py-2 rounded mr-2 hover:bg-yellow-600"
                    disabled={loading}
                  >
                    Edit
                  </button>
                  <button
                    onClick={() => handleDelete(todo.id)}
                    className="bg-red-500 text-white px-4 py-2 rounded hover:bg-red-600"
                    disabled={loading}
                  >
                    Delete
                  </button>
                </div>
              </div>
            )}
          </div>
        ))}
      </div>
    </div>
  );
}

export default App;