use asynq::error::Result;
use asynq::task::Task;

pub fn stub_traffic_statistics(task: Task) -> Result<()> {
    tracing::warn!("STUB traffic::statistics — task={}", task.get_type());
    Ok(())
}

pub fn stub_server_data(task: Task) -> Result<()> {
    tracing::warn!("STUB traffic::server_data — task={}", task.get_type());
    Ok(())
}

pub fn stub_reset_traffic(task: Task) -> Result<()> {
    tracing::warn!("STUB traffic::reset_traffic — task={}", task.get_type());
    Ok(())
}

pub fn stub_traffic_stat(task: Task) -> Result<()> {
    tracing::warn!("STUB traffic::stat — task={}", task.get_type());
    Ok(())
}
