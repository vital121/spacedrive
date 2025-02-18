import { useMemo } from 'react';
import { stringify } from 'uuid';
import {
	CRDTOperation,
	CRDTOperationData,
	useLibraryMutation,
	useLibraryQuery,
	useLibrarySubscription
} from '@sd/client';
import { Button } from '@sd/ui';
import { useRouteTitle } from '~/hooks/useRouteTitle';

type MessageGroup = {
	model: string;
	id: string;
	messages: { data: CRDTOperationData; timestamp: number }[];
};

export const Component = () => {
	useRouteTitle('Sync');

	const messages = useLibraryQuery(['sync.messages']);
	const backfillSyncMessages = useLibraryMutation(['sync.backfill'], {
		onSuccess: () => messages.refetch()
	});

	useLibrarySubscription(['sync.newMessage'], {
		onData: () => messages.refetch()
	});

	const groups = useMemo(
		() => (messages.data && calculateGroups(messages.data)) || [],
		[messages]
	);

	return (
		<ul className="space-y-4 p-4">
			<Button
				variant="accent"
				onClick={() => backfillSyncMessages.mutate(null)}
				disabled={backfillSyncMessages.isLoading}
			>
				Backfill Sync Messages
			</Button>
			{groups?.map((group, index) => <OperationGroup key={index} group={group} />)}
		</ul>
	);
};

const OperationGroup = ({ group }: { group: MessageGroup }) => {
	const [header, contents] = (() => {
		const header = (
			<div className="flex items-center space-x-2 p-2">
				<span>{group.model}</span>
				<span className="">{group.id}</span>
			</div>
		);
		const contents = (
			<ul className="flex flex-col space-y-2 p-2">
				{group.messages.map((message, index) => (
					<li key={index} className="flex flex-row justify-between px-2">
						{typeof message.data === 'string' ? (
							<p>{message.data === 'c' ? 'Create' : 'Delete'}</p>
						) : (
							<p>Update - {message.data.u.field}</p>
						)}
						<p className="text-gray-400">{message.timestamp}</p>
					</li>
				))}
			</ul>
		);
		return [header, contents];
	})();

	return (
		<div className="divide-y divide-gray bg-app-darkBox">
			{header}
			{contents}
		</div>
	);
};

function calculateGroups(messages: CRDTOperation[]) {
	return messages.reduce<MessageGroup[]>((acc, op) => {
		const { data } = op;

		const id = JSON.stringify(op.record_id);

		const latest = (() => {
			const latest = acc[acc.length - 1];

			if (!latest || latest.model !== op.model || latest.id !== id) {
				const group: MessageGroup = {
					model: op.model,
					id,
					messages: []
				};

				acc.push(group);

				return group;
			} else return latest;
		})();

		latest.messages.push({
			data,
			timestamp: op.timestamp
		});

		return acc;
	}, []);
}
