<script>
    import {Chess} from 'svelte-chess';
    let chess;
</script>
<Chess bind:this={chess}/>
<button on:click={()=>chess?.reset()}>Reset</button>
<button on:click={()=>chess?.undo()}>Undo</button>