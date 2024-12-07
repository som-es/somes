<script lang="ts">
	import { popup, type PopupSettings } from '@skeletonlabs/skeleton';
    import { onMount } from 'svelte';
    let messages = [
        { type: 'received', content: 'Hello! How can I help you today?' },
        { type: 'sent', content: 'Hi! Can you tell me more about your services?' },
        { type: 'received', content: 'Of course! We offer a range of solutions tailored to your needs.' }
    ];
    let newMessage = '';

    const sendMessage = () => {
        if (newMessage.trim()) {
            messages = [...messages, { type: 'sent', content: newMessage.trim() }];
            newMessage = '';
        }
    };
    
    const popupFeatured: PopupSettings = {
		event: 'hover',
		target: 'extractedFromIntroductionVideo',
		placement: 'bottom'
	};
</script>
