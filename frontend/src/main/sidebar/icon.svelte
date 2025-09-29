<script lang="ts">
    let {
        icon,
        text,
        selected,
        onClicked,
    }: { icon: string; text: string; selected: boolean, onClicked: () => void } = $props();
</script>

{#if selected}
    <button
        onclick={onClicked}
        class="flex flex-col items-center justify-center h-16 bg-zinc-700 shadow-[inset_-3px_0_0_0_#00bc7d] cursor-pointer transition"
    >
        <span class="material-symbols-outlined">{icon}</span>
        <p class="text-xs">{text}</p>
    </button>
{:else}
    <button
        onclick={onClicked}
        class="flex flex-col items-center justify-center h-16 hover:bg-zinc-700 cursor-pointer transition"
    >
        <span class="material-symbols-outlined">{icon}</span>
        <p class="text-xs">{text}</p>
    </button>
{/if}

<style>
</style>
