<script lang="ts">
    const id = $props.id();
    let { value = $bindable("") }: { value: string } = $props();

    const presets = [
        "#3B82F6", "#EF4444", "#F97316", "#F59E0B",
        "#EAB308", "#84CC16", "#22C55E", "#10B981",
        "#14B8A6", "#06B6D4", "#0EA5E9", "#6366F1",
        "#8B5CF6", "#A855F7", "#D946EF", "#EC4899",
        "#F43F5E", "#78716C", "#64748B", "#6B7280",
        "#DC2626", "#16A34A", "#2563EB", "#9333EA",
    ];

    let customInput: HTMLInputElement;

    function selectPreset(hex: string) {
        value = hex;
    }

    function openCustomPicker() {
        customInput?.click();
    }
</script>

<div class="flex flex-wrap gap-2">
    {#each presets as hex (hex)}
        <button
            type="button"
            aria-label="Select color {hex}"
            onclick={() => selectPreset(hex)}
            class="h-6 w-6 rounded-full border-2 cursor-pointer transition-transform hover:scale-110"
            style="background-color: {hex}; border-color: {value === hex ? '#000' : 'transparent'};"
        ></button>
    {/each}

    <button
        type="button"
        aria-label="Pick custom color"
        onclick={openCustomPicker}
        class="h-6 w-6 rounded-full border-2 border-dashed border-gray-400 flex items-center justify-center text-xs cursor-pointer"
        style="border-color: {!presets.includes(value) ? '#000' : undefined};"
    >
        +
    </button>
    <input
        bind:this={customInput}
        id="{id}-color"
        type="color"
        bind:value
        class="sr-only"
    />
</div>