<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { pickDirectory } from "$lib/fileDialog";

  let name = "";
  let results: Array<String> = [];

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    results = await invoke("search", { name }) as Array<String>;
  }

  async function index_files() {
    const dir = await pickDirectory();
    await invoke("index", { root: dir });
  }
</script>

<div class="container">
  <h1>Search</h1>
  <div class="row">
    <button on:click={index_files}>Index Files</button>
  </div>

  <br>

  <input id="greet-input" placeholder="Enter a name..." bind:value={name} on:input={greet}/>

  <p>
  {#each results as item}
    {item} <br>
  {/each}
  </p>

</div>

<style>

  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: #0f0f0f;
    background-color: #f6f6f6;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }

  .container {
    margin: 0;
    padding-top: 10vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: center;
  }

  .row {
    display: flex;
    justify-content: center;
  }

  h1 {
    text-align: center;
  }

  input,
  button {
    border-radius: 8px;
    border: 1px solid transparent;
    padding: 0.6em 1.2em;
    font-size: 1em;
    font-weight: 500;
    font-family: inherit;
    color: #0f0f0f;
    background-color: #ffffff;
    transition: border-color 0.25s;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
  }

  button {
    cursor: pointer;
  }

  button:hover {
    border-color: #396cd8;
  }
  button:active {
    border-color: #396cd8;
    background-color: #e8e8e8;
  }

  input,
  button {
    outline: none;
  }

  #greet-input {
    margin-right: 5px;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      color: #f6f6f6;
      background-color: #2f2f2f;
    }

    input,
    button {
      color: #ffffff;
      background-color: #0f0f0f98;
    }
    button:active {
      background-color: #0f0f0f69;
    }
  }
</style>
