<!-- Test file -->

<script>
    import { invoke } from "@tauri-apps/api/tauri";
    let path = "";
    // 

    async function git_init() {
        await invoke("init", { path });
    }
</script>



<div>
    <form class="row" on:submit|preventDefault={git_init}>
      <input id="greet-input" placeholder="Enter a path..." bind:value={path} />
      <button type="submit">Greet</button>
    </form>
</div>
