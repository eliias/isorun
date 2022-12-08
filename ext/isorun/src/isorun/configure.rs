use crate::js::worker::WORKER;
use magnus::block::Proc;

pub(crate) fn set_receiver(receiver: Option<Proc>) {
    WORKER.with(|worker| match receiver {
        None => worker.ruby_receiver.replace(None),
        Some(r) => worker.ruby_receiver.replace(Some(r)),
    });
}
